use serde_json::Value;

use super::express::{logic_combine, Express, FieldExpress, JsonObject, Predicate, Predicates};

use super::operators::{
    validate_compare_operator, validate_logic_operator, CompareOperator, LogicOperator,
};

use super::error::*;

fn decode_json_predicates(obj: &JsonObject) -> Predicates {
    let mut predicates: Predicates = Vec::with_capacity(obj.len());
    for (operator, value) in obj.iter() {
        if let Some(comp) = validate_compare_operator(operator) {
            predicates.push(Predicate {
                op: comp,
                value: value.clone(),
            })
        }
    }
    predicates
}

fn decode_json_object(obj: &JsonObject) -> Result<Express, ExpressError> {
    let mut results: Vec<Express> = Vec::with_capacity(obj.len());
    for (key, value) in obj.iter() {
        if let Some(_) = validate_compare_operator(key) {
            // {$op: xxx} must be predicates
            return Err(ExpressError::ValueError);
        }

        if let Some(op) = validate_logic_operator(key) {
            // login value must predicates array
            if let Value::Array(arr) = value {
                let items = decode_json_array(arr)?;
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
                        value: value.clone(),
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
fn decode_json_array(arr: &Vec<Value>) -> Result<Vec<Express>, ExpressError> {
    let mut express: Vec<Express> = vec![];
    for i in arr {
        express.push(decode_express(i)?)
    }
    Ok(express)
}

pub fn decode_express(value: &Value) -> Result<Express, ExpressError> {
    match value {
        Value::Array(arr) => Ok(logic_combine(LogicOperator::AND, decode_json_array(arr)?)),
        Value::Object(o) => decode_json_object(o),
        _ => Err(ExpressError::ValueError),
    }
}
