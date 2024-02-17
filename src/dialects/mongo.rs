use std::collections::HashMap;

use pyo3::{PyAny, PyObject, Python, ToPyObject};

use crate::query::express::{
    CompareOperator, Express, FieldExpress, LogicExpress, LogicOperator, Predicate,
};

fn mongo_predicates(py: Python, predicates: Vec<Predicate<&PyAny>>) -> PyObject {
    let mut dict: HashMap<String, &PyAny> = HashMap::new();
    for Predicate { op, value } in predicates {
        let key = match op {
            CompareOperator::EQ => "$eq".into(),
            CompareOperator::NE => "$ne".into(),
            CompareOperator::GE => "$ge".into(),
            CompareOperator::GT => "$gt".into(),
            CompareOperator::LE => "$le".into(),
            CompareOperator::LT => "$lt".into(),
            CompareOperator::IN => "$in".into(),
            CompareOperator::NIN => "$nin".into(),
            CompareOperator::REGEX => "$regex".into(),
        };
        dict.insert(key, *value);
    }
    dict.to_object(py)
}
fn mongo_field(py: Python, expr: FieldExpress<&PyAny>) -> PyObject {
    let mut dict: HashMap<String, &PyAny> = HashMap::new();
    let binding = mongo_predicates(py, expr.predicates);
    dict.insert(expr.field, binding.as_ref(py));
    dict.to_object(py)
}

fn mongo_logic(py: Python, expr: LogicExpress<&PyAny>) -> PyObject {
    let mut list: Vec<&PyAny> = Vec::new();
    for exp in expr.express {
        let t = match exp {
            Express::Field(ex) => mongo_field(py, ex),
            Express::Logic(ex) => mongo_logic(py, ex),
        };
        list.push(t.into_ref(py));
    }
    let mut dict: HashMap<String, &PyAny> = HashMap::new();
    let key = match expr.op {
        LogicOperator::AND => "$and".into(),
        LogicOperator::NOT => "$not".into(),
        LogicOperator::OR => "$or".into(),
        LogicOperator::NOR => "$nor".into(),
    };
    dict.insert(key, list.to_object(py).into_ref(py));
    dict.to_object(py)
}

#[allow(unused)]
pub fn mongo(py: Python, express: Express<&PyAny>) -> PyObject {
    match express {
        Express::Field(expr) => mongo_field(py, expr),
        Express::Logic(expr) => mongo_logic(py, expr),
    }
}
