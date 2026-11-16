use pyo3::exceptions::{PyKeyError, PySyntaxError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};
use pyo3::IntoPyObjectExt;

use t3d::{T3dObject, T3dPropertyValue, T3dReference, T3dStruct, T3dValue};

/// A reference to another object, e.g. `StaticMesh'Package.Group.Name'`.
#[pyclass(name = "T3dReference")]
#[derive(Clone)]
struct PyT3dReference(T3dReference);

#[pymethods]
impl PyT3dReference {
    #[getter]
    fn type_(&self) -> &str {
        &self.0.type_
    }

    #[getter]
    fn path(&self) -> &str {
        &self.0.path
    }

    fn __repr__(&self) -> String {
        self.0.to_string()
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }
}

/// An object (`Begin <Type> ... End <Type>`) in a T3D file.
#[pyclass(name = "T3dObject")]
struct PyT3dObject(T3dObject);

#[pymethods]
impl PyT3dObject {
    #[getter]
    fn type_(&self) -> &str {
        &self.0.type_
    }

    #[getter]
    fn children<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        PyList::new(
            py,
            self.0
                .children
                .iter()
                .map(|child| PyT3dObject(child.as_ref().clone())),
        )
    }

    #[getter]
    fn properties<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        for (name, value) in &self.0.properties {
            dict.set_item(name, property_value_to_py(py, value)?)?;
        }
        Ok(dict)
    }

    #[getter]
    fn vector_properties<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.0.vector_properties.clone().into_bound_py_any(py)
    }

    fn __getitem__<'py>(&self, py: Python<'py>, key: &str) -> PyResult<Bound<'py, PyAny>> {
        match self.0.get(key) {
            Some(value) => property_value_to_py(py, value),
            None => Err(PyKeyError::new_err("Property not found")),
        }
    }
}

fn struct_to_py<'py>(py: Python<'py>, value: &T3dStruct) -> PyResult<Bound<'py, PyAny>> {
    let dict = PyDict::new(py);
    for (name, value) in value {
        dict.set_item(name, value_to_py(py, value)?)?;
    }
    Ok(dict.into_any())
}

fn value_to_py<'py>(py: Python<'py>, value: &T3dValue) -> PyResult<Bound<'py, PyAny>> {
    match value {
        T3dValue::Int(value) => value.into_bound_py_any(py),
        T3dValue::Float(value) => value.into_bound_py_any(py),
        T3dValue::Bool(value) => value.into_bound_py_any(py),
        T3dValue::String(value) => value.into_bound_py_any(py),
        T3dValue::Struct(value) => struct_to_py(py, value),
        T3dValue::InlineStruct(value) => struct_to_py(py, value),
        T3dValue::Reference(value) => PyT3dReference(value.clone()).into_bound_py_any(py),
        T3dValue::Identifier(value) => value.into_bound_py_any(py),
        T3dValue::Vector(value) => value.into_bound_py_any(py),
        T3dValue::Array(values) => {
            let items = values
                .iter()
                .map(|value| match value {
                    Some(value) => value_to_py(py, value),
                    None => Ok(py.None().into_bound(py)),
                })
                .collect::<PyResult<Vec<_>>>()?;
            PyList::new(py, items)?.into_bound_py_any(py)
        }
    }
}

fn property_value_to_py<'py>(
    py: Python<'py>,
    value: &T3dPropertyValue,
) -> PyResult<Bound<'py, PyAny>> {
    match value {
        T3dPropertyValue::Value(value) => value_to_py(py, value),
        T3dPropertyValue::Array(values) => {
            let items = values
                .iter()
                .map(|(index, value)| {
                    PyTuple::new(py, [index.into_bound_py_any(py)?, value_to_py(py, value)?])
                })
                .collect::<PyResult<Vec<_>>>()?;
            PyList::new(py, items)?.into_bound_py_any(py)
        }
    }
}

#[pyfunction]
fn read_t3d(contents: &str) -> PyResult<Vec<PyT3dObject>> {
    t3d::parse_t3d(contents)
        .map(|objects| objects.into_iter().map(PyT3dObject).collect())
        .map_err(|err| PySyntaxError::new_err(format!("{:?}", err)))
}

#[pymodule]
fn t3dpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyT3dObject>()?;
    m.add_class::<PyT3dReference>()?;
    m.add_function(wrap_pyfunction!(read_t3d, m)?)?;
    Ok(())
}
