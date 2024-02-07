use serde_json::Value;


use super::express::{logic_combine, Express, FieldExpress, JsonObject, Predicate, Predicates};

use super::operators::{
    validate_compare_operator, validate_logic_operator, CompareOperator, LogicOperator,
};

fn decode_json_predicates(obj: JsonObject) -> Predicates {
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

fn decode_json_object(obj: JsonObject) -> Express {
    let mut results: Vec<Express> = Vec::with_capacity(obj.len());
    for (key, value) in obj.iter() {
        if let Some(op) = validate_logic_operator(key) {
            // login value must predicates array
            if let Value::Array(arr) = value {
                let items = decode_json_array(arr.clone());
                results.push(logic_combine(op, items));
            }
        } else if let Some(_) = validate_compare_operator(key) {
            // {$op: xxx} must be predicates
            panic!()
        } else {
            // key is field
            match value {
                Value::Object(obj) => {
                    // 1.value is predicates
                    // 2.value has $type
                    // 3.value is doc
                    results.push(Express::Field(FieldExpress {
                        field: String::from(key),
                        predicates: decode_json_predicates(obj.clone()),
                    }))
                }
                _ => {
                    let mut predicates: Predicates = Vec::with_capacity(1);
                    predicates.push(Predicate {
                        op: CompareOperator::EQ,
                        value: value.clone(),
                    });

                    results.push(Express::Field(FieldExpress {
                        field: String::from(key),
                        predicates: predicates,
                    }))
                }
            }
        }
    }
    if results.len() > 1 {
        logic_combine(LogicOperator::AND, results)
    } else {
        results.pop().unwrap()
    }
}
fn decode_json_array(arr: Vec<Value>) -> Vec<Express> {
    let express: Vec<Express> = arr
        .into_iter()
        .map(decode_express)
        .collect();

    express
}

pub fn decode_express(value: Value) -> Express {
    match value {
        Value::Array(arr) => logic_combine(LogicOperator::AND, decode_json_array(arr)),
        Value::Object(o) => decode_json_object(o),
        _ => panic!("value error"),
    }
}
