extern crate pest;

use std::collections::HashMap;
use std::fmt::{Debug};
use pest_consume::Parser;
use pest_consume::{Error, match_nodes};
use crate::ast::*;

#[derive(Parser)]
#[grammar = "t3d.pest"]
pub struct T3dParser;

type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

macro_rules! match_nodes_any {
    ($nodes:expr; $($f:ident($v:ident) => $e:expr),*) => (
        for node in $nodes {
            match node.as_rule() {
                $(
                    Rule::$f => {
                        let $v = Self::$f(node)?;   // TODO: exclude this if the var isn't defined!
                        $e
                    }
                )+
                _ => { return Err(node.error("unexpected rule")) }
            }
        }
    )
}

#[pest_consume::parser]
impl T3dParser {

    fn id(input: Node) -> Result<String> {
        return Ok(String::from(input.as_str()))
    }

    fn object_statements(input: Node) -> Result<Vec<T3dObjectStatement>> {
        let mut statements = Vec::new();
        match_nodes_any!(input.into_children();
            object_statement(s) => statements.push(s)
        );
        Ok(statements)
    }

    fn object(input: Node) -> Result<T3dObject> {
        let mut children = Vec::new();
        let mut properties = HashMap::new();
        let mut vector_properties = Vec::new();
        match_nodes!(input.into_children();
            [id(i), object_statements(statements), id(_)] => {
                for statement in statements {
                    match statement {
                        T3dObjectStatement::Object(o) => {
                            children.push(Box::new(o));
                        },
                        T3dObjectStatement::PropertyAssignment(p) => {
                            if let Some(value) = properties.get_mut(&p.name) {
                                // Property has already been encountered.
                                match value {
                                    T3dPropertyValue::Array(a) => {
                                        a.push((p.index, p.value))
                                    },
                                    T3dPropertyValue::Value(_) => {
                                        // TODO: handle this gracefully, for now just ignore
                                        //return Err(format!("Un-indexed property assignment encountered for array type {}", &p.name))
                                    }
                                }
                            } else {
                                // New property encountered.
                                if let Some(_) = p.index {
                                    // Property is an array, so add a new array type.
                                    properties.insert(p.name, T3dPropertyValue::Array(vec![(p.index, p.value)]));
                                } else {
                                    properties.insert(p.name, T3dPropertyValue::Value(p.value));
                                }
                            }
                        },
                        T3dObjectStatement::PropertyAssignmentVector(p) => {
                            vector_properties.push((p.name, p.value))
                        }
                    }
                }
                Ok(T3dObject {
                    type_: i,
                    children,
                    properties,
                    vector_properties
                })
            },
            [id(i), id(_)] => {
                Ok(T3dObject {
                    type_: i,
                    children,
                    properties,
                    vector_properties
                })
            }
        )
    }

    fn object_statement(input: Node) -> Result<T3dObjectStatement> {
        match_nodes!(input.into_children();
            [object(o)] => Ok(T3dObjectStatement::Object(o)),
            [property_assignment(p)] => Ok(T3dObjectStatement::PropertyAssignment(p)),
            [property_assignment_vector(p)] => Ok(T3dObjectStatement::PropertyAssignmentVector(p)),
        )
    }

    fn property_assignment(input: Node) -> Result<T3dPropertyAssignment> {
        // An empty string will be written like this:
        // Foo=
        // Instead of:
        // Foo=""
        // Therefore, we have to report a missing value as an empty string.
        match_nodes!(input.into_children();
            [id(name), value(v)] => Ok(T3dPropertyAssignment {
                name,
                value: v,
                index: None
            }),
            [id(name), int(index), value(v)] => Ok(T3dPropertyAssignment {
                name,
                value: v,
                index: Some(index)
            }),
            [id(name)] => Ok(T3dPropertyAssignment {
                name,
                value: T3dValue::String(String::new()),
                index: None
            }),
            [id(name), int(index)] => Ok(T3dPropertyAssignment {
                name,
                value: T3dValue::String(String::new()),
                index: Some(index)
            }),
        )
    }

    fn property_assignment_vector(input: Node) -> Result<T3dPropertyAssignmentVector> {

        match_nodes!(input.into_children();
            [id(name), float(x), float(y), float(z)] => {
                Ok(T3dPropertyAssignmentVector {
                    name,
                    value: (x, y, z),
                })
            }
        )
    }

    fn int(input: Node) -> Result<i32> {
        Ok(input.as_str().parse::<i32>().unwrap())
    }

    fn float(input: Node) -> Result<f32> {
        Ok(input.as_str().parse::<f32>().unwrap())
    }

    fn string(input: Node) -> Result<String> {
        Ok(input.as_str().to_string())
    }

    fn struct_(input: Node) -> Result<Box<T3dStruct>> {
        let mut properties = HashMap::new();
        match_nodes_any!(input.into_children();
            property_assignment(p) => {
                properties.insert(p.name, p.value);
            }
        );
        Ok(Box::new(properties))
    }

    fn reference_path(input: Node) -> Result<String> {
        Ok(String::from(input.as_str()))
    }

    fn reference(input: Node) -> Result<T3dReference> {
        match_nodes!(input.into_children();
            [id(type_), reference_path(path)] => Ok(T3dReference { type_, path })
        )
    }

    fn array(input: Node) -> Result<Vec<Option<T3dValue>>> {
        let mut values: Vec<Option<T3dValue>> = Vec::new();
        match_nodes_any!(input.into_children();
            value(v) => values.push(Some(v))
        );
        Ok(values)
    }

    fn value(input: Node) -> Result<T3dValue> {
        let str = String::from(input.as_str());
        match_nodes!(input.into_children();
            [array(a)] => Ok(T3dValue::Array(a)),
            [int(i)] => Ok(T3dValue::Int(i)),
            [float(f)] => Ok(T3dValue::Float(f)),
            [string(s)] => Ok(T3dValue::String(s)),
            [struct_(s)] => Ok(T3dValue::Struct(s)),
            [reference(r)] => Ok(T3dValue::Reference(r)),
            [id(i)] => {
                if str.eq_ignore_ascii_case("true") || str.eq_ignore_ascii_case("false") {
                    Ok(T3dValue::Bool(str.to_lowercase().parse::<bool>().unwrap()))
                } else {
                    Ok(T3dValue::Identifier(i))
                }
            }
        )
    }

    fn EOI(input: Node) -> Result<()> {
        Ok(())
    }

    fn t3d(input: Node) -> Result<Vec<T3dObject>> {
        let mut objects = Vec::new();
        match_nodes_any!(input.into_children();
            object(o) => objects.push(o),
            EOI(_e) => {}
        );
        Ok(objects)
    }
}

pub type T3dSyntaxError = Error<Rule>;

pub fn parse_t3d(contents: &str) -> std::result::Result<Vec<T3dObject>, T3dSyntaxError> {
    match T3dParser::t3d(T3dParser::parse(Rule::t3d, contents)?.single()?) {
        Ok(t3d) => { Ok(t3d) },
        Err(error) => Err(error)
    }
}
