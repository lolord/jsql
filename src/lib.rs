use pyo3::prelude::*;
use pyo3::types::PyTuple;

use std::collections::HashMap;

use serde_json::Value;

pub mod errors;
pub mod query;

pub mod dialects {
    pub mod mysql;
}

pub fn get_version() -> &'static str {
    return &"0.1.0";
}

fn py_object(py: Python, value: &Value) -> Py<PyAny> {
    match value {
        Value::Bool(b) => b.to_object(py),
        Value::Null => ().to_object(py),
        Value::String(s) => s.to_object(py),
        Value::Number(n) => {
            if n.is_i64() {
                n.as_i64().unwrap().to_object(py)
            } else if n.is_u64() {
                n.as_i64().unwrap().to_object(py)
            } else {
                n.as_f64().unwrap().to_object(py)
            }
        }
        Value::Array(arr) => PyTuple::new(py, arr.iter().map(|x| py_object(py, x))).into_py(py),
        Value::Object(obj) => {
            let dict: HashMap<&String, Py<PyAny>> =
                obj.iter().map(|(k, v)| (k, py_object(py, v))).collect();
            dict.to_object(py)
        }
    }
}

#[pyfunction]
fn mysql(py: Python, value: String) -> PyResult<(String, PyObject)> {
    let json_result: Result<Value, _> = serde_json::from_str(&value);
    match json_result {
        Ok(json) => match query::decode::decode_express(&json) {
            Ok(expr) => {
                let (sql, params) = dialects::mysql::mysql(expr);
                let args = PyTuple::new(py, params.iter().map(|x| py_object(py, x))).into_py(py);
                Ok((sql, args))
            }
            Err(e) => return Err(errors::py_error(e)),
        },
        Err(e) => return Err(errors::py_error(e)),
    }
}

#[pyfunction]
fn mongo(py: Python, value: String) -> PyResult<(String, PyObject)> {
    println!("obj{:?}", value);

    let json_result: Result<Value, _> = serde_json::from_str(&value);
    match json_result {
        Ok(json) => match query::decode::decode_express(&json) {
            Ok(expr) => {
                let (sql, params) = dialects::mysql::mysql(expr);
                let args = PyTuple::new(py, params.iter().map(|x| py_object(py, x))).into_py(py);
                Ok((sql, args))
            }
            Err(e) => return Err(errors::py_error(e)),
        },
        Err(e) => return Err(errors::py_error(e)),
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
#[pyo3(name = "_jsql")]
fn jsql(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__version__", get_version())?;
    m.add_function(wrap_pyfunction!(mysql, m)?)?;
    m.add_function(wrap_pyfunction!(mongo, m)?)?;
    Ok(())
}
