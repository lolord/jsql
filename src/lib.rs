use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString, PyTuple};

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

// ptdantic_core/src/lib.rs
pub fn from_json(
    py: Python,
    data: &PyAny,
    allow_inf_nan: bool,
    cache_strings: bool,
) -> PyResult<PyObject> {
    match data.downcast::<PyString>() {
        Ok(py_str) => {
            let x = py_str.to_string_lossy();
            let json_bytes: &[u8] = x.as_bytes();
            jiter::python_parse(py, json_bytes, allow_inf_nan, cache_strings)
                .map_err(|e| jiter::map_json_error(json_bytes, &e))
        }
        Err(e) => return Err(errors::py_error(e)),
    }
}

#[pyfunction]
fn mysql(py: Python, value: &PyAny) -> PyResult<(String, PyObject)> {
    let py_value = if let Ok(py_str) = value.downcast::<PyString>() {
        from_json(py, py_str, false, false).unwrap().into_ref(py)
    } else {
        value
    };
    match py_value.downcast::<PyDict>() {
        Ok(py_dict) => match query::decode_python::decode_express(py_dict) {
            Ok(expr) => {
                let (sql, params) = dialects::mysql::mysql(expr);
                let args = PyTuple::new(py, params).into_py(py);
                return Ok((sql, args));
            }
            Err(e) => return Err(errors::py_error(e)),
        },
        Err(e) => return Err(errors::py_error(e)),
    }
}

// #[pyfunction]
// fn mongo(py: Python, value: String) -> PyResult<(String, PyObject)> {
//     println!("obj{:?}", value);

//     let json_result: Result<Value, _> = serde_json::from_str(&value);
//     match json_result {
//         Ok(json) => match query::decode::decode_express(&json) {
//             Ok(expr) => {
//                 let (sql, params) = dialects::mysql::mysql(expr);
//                 let args = PyTuple::new(py, params.iter().map(|x| py_object(py, x))).into_py(py);
//                 Ok((sql, args))
//             }
//             Err(e) => return Err(errors::py_error(e)),
//         },
//         Err(e) => return Err(errors::py_error(e)),
//     }
// }

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
#[pyo3(name = "_jsql")]
fn jsql(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__version__", get_version())?;
    m.add_function(wrap_pyfunction!(mysql, m)?)?;
    // m.add_function(wrap_pyfunction!(mongo, m)?)?;
    Ok(())
}
