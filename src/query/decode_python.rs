use std::borrow::Cow;

use super::express::{logic_combine, Express, FieldExpress, Predicate, Predicates};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyString};

use super::operators::{
    validate_compare_operator, validate_logic_operator, CompareOperator, LogicOperator,
};

use super::error::ExpressError;

fn get_py_string(key: &PyAny) -> Result<Cow<'_, str>, ExpressError> {
    if let Ok(py_str) = key.downcast::<PyString>() {
        return Ok(py_str.to_string_lossy());
    }
    return Err(ExpressError::ValueError("key is not str".into()));
}

fn decode_py_predicates(obj: &PyDict) -> Result<Predicates<&PyAny>, ExpressError> {
    let mut predicates = Vec::with_capacity(obj.len());
    for (key, value) in obj.iter() {
        let operator: String = get_py_string(key)?.into();
        if let Some(comp) = validate_compare_operator(&operator) {
            predicates.push(Predicate {
                op: comp,
                value: Box::new(value),
            })
        }
    }
    Ok(predicates)
}

fn decode_py_dict(py_dict: &PyDict) -> Result<Express<&PyAny>, ExpressError> {
    let mut results = Vec::with_capacity(py_dict.len());
    for (py_key, value) in py_dict {
        let key: String = get_py_string(py_key)?.into();

        if let Some(op) = validate_logic_operator(&key) {
            // login value must predicates array
            // if let Value::Array(arr) = value {
            if let Ok(py_list) = value.downcast::<PyList>() {
                let items = decode_py_list(py_list)?;
                results.push(logic_combine(op, items));
            }
        } else {
            // key is field
            if let Ok(dict) = value.downcast::<PyDict>() {
                // 1.value is predicates
                // 2.value has $type
                // 3.value is doc
                results.push(Express::Field(FieldExpress {
                    field: key,
                    predicates: decode_py_predicates(dict)?,
                }))
            } else {
                results.push(Express::Field(FieldExpress {
                    field: String::from(key),
                    predicates: vec![Predicate {
                        op: CompareOperator::EQ,
                        value: value.into(),
                    }],
                }))
            }
        }
    }
    if results.len() > 1 {
        Ok(logic_combine(LogicOperator::AND, results))
    } else {
        Ok(results.pop().unwrap())
    }
}
fn decode_py_list(py_list: &PyList) -> Result<Vec<Express<&PyAny>>, ExpressError> {
    let mut express: Vec<Express<&PyAny>> = vec![];
    for i in py_list {
        express.push(decode_express(i)?)
    }
    Ok(express)
}

pub fn decode_express(value: &PyAny) -> Result<Express<&PyAny>, ExpressError> {
    match value.downcast::<PyDict>() {
        Ok(py_dict) => decode_py_dict(py_dict),
        _ => Err(ExpressError::ValueError("ValueError".into())),
    }
}
