use super::express::{logic_combine, Express, FieldExpress, Predicate, Predicates};

use super::operators::{
    validate_compare_operator, validate_logic_operator, CompareOperator, LogicOperator,
};

use super::error::ExpressError;
use serde_json::{Map, Value};
pub type JsonObject<T> = Map<String, T>;

fn decode_json_predicates(obj: Map<String, Value>) -> Predicates<Value> {
    let mut predicates: Predicates<Value> = Vec::with_capacity(obj.len());
    for (operator, value) in obj {
        if let Some(comp) = validate_compare_operator(&operator) {
            predicates.push(Predicate {
                op: comp,
                value: Box::new(value),
            })
        }
    }
    predicates
}

fn decode_json_object(obj: Map<String, Value>) -> Result<Express<Value>, ExpressError> {
    let mut results: Vec<Express<Value>> = Vec::new();
    for (key, value) in obj {
        if let Some(_) = validate_compare_operator(&key) {
            return Err(ExpressError::ValueError(
                "{$op: xxx} must be predicates".into(),
            ));
        }

        if let Some(op) = validate_logic_operator(&key) {
            // login value must predicates array
            if let Value::Array(arr) = value {
                let items = decode_json_array(&arr)?;
                results.push(logic_combine(op, items));
            }
        } else {
            // key is field
            match value {
                Value::Object(obj) => {
                    // 1.value is predicates
                    // 2.value has $type
                    // 3.value is doc
                    results.push(Express::Field(FieldExpress {
                        field: String::from(key),
                        predicates: decode_json_predicates(obj),
                    }))
                }
                _ => results.push(Express::Field(FieldExpress {
                    field: String::from(key),
                    predicates: vec![Predicate {
                        op: CompareOperator::EQ,
                        value: value.into(),
                    }],
                })),
            }
        }
    }
    if results.len() > 1 {
        Ok(logic_combine(LogicOperator::AND, results))
    } else {
        Ok(results.pop().unwrap())
    }
}
fn decode_json_array(arr: &Vec<Value>) -> Result<Vec<Express<Value>>, ExpressError> {
    let mut express: Vec<Express<Value>> = vec![];
    for i in arr {
        express.push(decode_express(i)?)
    }
    Ok(express)
}

pub fn decode_express(value: &Value) -> Result<Express<Value>, ExpressError> {
    match value {
        Value::Array(arr) => Ok(logic_combine(LogicOperator::AND, decode_json_array(arr)?)),
        Value::Object(ref o) => decode_json_object(o.clone()),
        _ => Err(ExpressError::ValueError("ValueError".into())),
    }
}

pub fn decode_string_express(value: String) -> Result<Express<Value>, ExpressError> {
    match serde_json::from_str(&value) {
        Ok(json) => decode_express(&json),
        Err(_) => Err(ExpressError::ValueError("json".into())),
    }
}
