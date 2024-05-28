extern crate core;

mod parser;
mod ast;

use pyo3::prelude::*;

use pyo3::exceptions::PySyntaxError;
use crate::ast::{T3dObject, T3dReference};
use crate::parser::parse_t3d;

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
    m.add_class::<T3dReference>()?;
    m.add_function(wrap_pyfunction!(read_t3d, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use crate::ast::{T3dPropertyValue, T3dValue};
    use super::*;

    #[test]
    fn property_assignment_with_reference_array() -> Result<(), String> {
        let contents = String::from("
        Begin Object
            SomeArray=(StaticMesh'Foo.Bar',StaticMesh'Baz.Boo')
        End Object
        ");
        let result = parser::parse_t3d(contents.as_str());
        match result {
            Ok(objects) => {
                let object = objects.first().unwrap();
                let property = object.properties.get("SomeArray").unwrap();
                match property {
                    T3dPropertyValue::Value(v) => {
                        match v {
                            T3dValue::Array(values) => {
                                match values.get(0).unwrap().clone().unwrap() {
                                    T3dValue::Reference(reference) => {
                                        assert_eq!(reference.type_, "StaticMesh");
                                        assert_eq!(reference.path, "Foo.Bar");
                                    }
                                    _ => {
                                        assert_eq!(false, true);
                                    }
                                };
                                match values.get(1).unwrap().clone().unwrap() {
                                    T3dValue::Reference(reference) => {
                                        assert_eq!(reference.type_, "StaticMesh");
                                        assert_eq!(reference.path, "Baz.Boo");
                                    }
                                    _ => {
                                        assert_eq!(false, true);
                                    }
                                }
                                Ok(())
                            },
                            _ => {
                                Err(String::from("Expected array value"))
                            }
                        }
                    },
                    T3dPropertyValue::Array(_) => {
                        Err(String::from("Wrong value type"))
                    }
                }
            },
            Err(error) => {
                Err(error.to_string())
            }
        }
    }

    #[test]
    fn property_assignment_empty_value() -> Result<(), String> {
        let contents = String::from("
            Begin Object
                MyEmptyString=
                TheNextString=\"\"
            End Object
        ");
        let result = parser::parse_t3d(contents.as_str());
        match &result {
            Ok(objects) => {
                assert_eq!(1, objects.len());
                let object = objects.first().unwrap();
                assert_eq!(2, object.properties.len());
                let value = object.properties.get("MyEmptyString");
                assert_eq!(true, value.is_some());
                match value.unwrap() {
                    T3dPropertyValue::Value(value) => {
                        match value {
                            T3dValue::String(string) => {
                                assert_eq!(string.as_str(), "");
                                Ok(())
                            }
                            _ => {
                                Err(String::from("Wrong type of value"))
                            }
                        }
                    }
                    _ => {
                        Err(String::from("Property value should have been a bare value (not an array)"))
                    }
                }
            }
            Err(error) => {
                Err(error.to_string())
            }
        }
    }

    #[test]
    fn it_works() -> Result<(), String> {
        let mut contents = String::new();
        match File::open("src/tests/data/terraininfo.t3d") {
            Ok(mut file) => {
                match file.read_to_string(&mut contents) {
                    Ok(_) => {
                        let result = parser::parse_t3d(contents.as_str());
                        match result {
                            Ok(_) => {
                                Ok(())
                            }
                            Err(error) => {
                                print!("{}", error.to_string().as_str());
                                Err("Syntax error".to_string())
                            }
                        }
                    }
                    Err(_) => {
                        Err("Failed to file contents".to_string())
                    }
                }
            }
            Err(_) => {
                Err("Failed to open file".to_string())
            }
        }
    }
}
