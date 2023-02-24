extern crate core;

mod parser;
mod ast;

use pyo3::prelude::*;

use std::fs::File;
use std::io::Read;
use pyo3::exceptions::{PySyntaxError};
use crate::ast::{T3dObject, T3dObjectStatement, T3dPropertyAssignment, T3dPropertyValue};
use crate::parser::{parse_t3d};

#[pyfunction]
fn read_t3d(contents: &str) -> PyResult<Vec<T3dObject>> {
    match parse_t3d(contents) {
        Ok(objects) => {
            Ok(objects)
        },
        Err(err) => {
            Err(PySyntaxError::new_err(format!("{:?}", err)))
        }
    }
}

#[pymodule]
fn t3dpy(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<T3dObject>()?;
    m.add_function(wrap_pyfunction!(read_t3d, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use super::*;

    #[test]
    fn it_works() {
        let mut contents = String::new();
        File::open("src/tests/data/terraininfo.t3d").unwrap().read_to_string(&mut contents);
        let result = parser::parse_t3d(contents.as_str());
        match result {
            Ok(objects) => {
                println!("{:?}", objects)
            }
            Err(error) => {
                println!("{:?}", error)
            }
        }
    }
}
