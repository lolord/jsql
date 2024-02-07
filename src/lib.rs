use pyo3::{exceptions::PyValueError, prelude::*};

#[allow(unused_imports)]
use core::ptr::{null, null_mut, NonNull};
use std::fmt::format;

use serde_json::Value;

pub mod query;

pub mod dialects {
    pub mod mysql;
}

pub fn get_version() -> &'static str {
    return &"0.1.0"
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn mysql(
    obj: String,
    )  -> PyResult<(String,Vec<String>)> {
    println!("obj{:?}",obj);

    let json_result: Result<Value,_> = serde_json::from_str(&obj);
    match json_result {
        Ok(json) => {
            let (sql, params) = dialects::mysql::mysql(query::decode::decode_express(json));
            println!("params {:?}",params);
            println!("params[0] {}",params[0]);
            println!("params[0] {}",format!("{}",params[0]) );
            // let x = serde_json::to_string(&(sql, params));
            Ok((sql, params.iter().map(|x|match x {
                Value::String(s) => s.to_owned(),
                other => format!("{}",other),
            }).collect()))
        },
        Err(e) => return  Err(PyValueError::new_err(e.to_string())),
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
    
    Ok(())
}    
