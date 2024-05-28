use std::collections::HashMap;
use std::fmt::{Debug};
use pyo3::prelude::*;
use pyo3::{IntoPy, PyObject, Python, ToPyObject};
use pyo3::exceptions::PyKeyError;

pub type T3dStruct = HashMap<String, T3dValue>;

// impl ToT3dString for T3dStruct {
//     fn to_t3d_string(&self) -> String {
//         let a = self.properties.iter().map(
//             |(key, value)| format!("{}={}", key, value.to_string())).collect::<Vec<String>>().join(",");
//         format!("({})", a)
//     }
// }

#[pyclass]
#[derive(Debug, Clone)]
pub struct T3dReference {
    #[pyo3(get)]
    pub type_: String,
    #[pyo3(get)]
    pub path: String,
}

impl ToString for T3dReference {
    fn to_string(&self) -> String {
        format!("{}'{}'", self.type_, self.path)
    }
}

#[pymethods]
impl T3dReference {
    fn __repr__(&self) -> String {
        self.to_string()
    }

    fn __str__(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug, Clone)]
pub enum T3dValue {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Struct(Box<T3dStruct>),
    Reference(T3dReference),
    Identifier(String),
    Vector((f32, f32, f32)),
    Array(Vec<Option<T3dValue>>)
}

impl ToString for T3dValue {
    fn to_string(&self) -> String {
        match self {
            T3dValue::Int(value) => value.to_string(),
            T3dValue::Float(value) => value.to_string(),
            T3dValue::Bool(value) => value.to_string(),
            T3dValue::String(value) => format!("\"{}\"", value.clone()),
            T3dValue::Struct(_) => String::from("Struct()"),
            T3dValue::Reference(value) => value.to_string(),
            T3dValue::Identifier(value) => value.clone(),
            T3dValue::Vector(value) => format!("{:?}", value.clone()),
            T3dValue::Array(_) => String::from("Array()"),
        }
    }
}

impl IntoPy<PyObject> for T3dValue {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            T3dValue::Int(value) => value.into_py(py),
            T3dValue::Float(value) => value.into_py(py),
            T3dValue::Bool(value) => value.into_py(py),
            T3dValue::String(value) => value.into_py(py),
            T3dValue::Struct(value) => value.as_ref().clone().into_py(py),
            T3dValue::Reference(value) => value.clone().into_py(py),
            T3dValue::Identifier(value) => value.into_py(py),
            T3dValue::Vector(value) => value.into_py(py),
            T3dValue::Array(value) => value.into_py(py),
        }
    }
}

impl ToPyObject for T3dValue {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        self.clone().into_py(py)
    }
}

#[derive(Debug, Clone)]
pub struct T3dPropertyAssignment {
    pub name: String,
    pub index: Option<i32>,
    pub value: T3dValue,
}

#[derive(Debug, Clone)]
pub struct T3dPropertyAssignmentVector {
    pub name: String,
    pub value: (f32, f32, f32),
}

#[derive(Debug, Clone)]
pub enum T3dPropertyValue {
    Value(T3dValue),
    Array(Vec<(Option<i32>, T3dValue)>),
}

impl IntoPy<PyObject> for T3dPropertyValue {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            T3dPropertyValue::Value(value) => { value.into_py(py) }
            T3dPropertyValue::Array(array) => { array.into_py(py) }

        }
    }
}

impl ToPyObject for T3dPropertyValue {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        self.clone().into_py(py)
    }
}

#[derive(Debug, Clone)]
pub enum T3dObjectStatement {
    Object(T3dObject),
    PropertyAssignment(T3dPropertyAssignment),
    PropertyAssignmentVector(T3dPropertyAssignmentVector)
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct T3dObject {
    #[pyo3(get)]
    pub type_: String,
    #[pyo3(get)]
    pub children: Vec<Box<T3dObject>>,
    #[pyo3(get)]
    pub properties: HashMap<String, T3dPropertyValue>,
    #[pyo3(get)]
    pub vector_properties: Vec<(String, (f32, f32, f32))>,
}

impl IntoPy<PyObject> for Box<T3dObject> {
    fn into_py(self, py: Python<'_>) -> PyObject {
        return self.as_ref().clone().into_py(py)
    }
}

#[pymethods]
impl T3dObject {
    fn __getitem__(&self, key: String) -> PyResult<T3dPropertyValue> {
        match self.properties.get(key.as_str()) {
            None => {
                Err(PyKeyError::new_err("Property not found"))
            }
            Some(property_value) => {
                Ok(property_value.clone())
            }
        }
    }
}
