use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyString, PyTuple};

pub mod errors;
pub mod query;

pub mod dialects {
    pub mod mongo;
    pub mod mysql;
}

pub fn get_version() -> &'static str {
    return &"0.1.0";
}

fn format_input<'a>(py: Python<'a>, input: &'a PyAny) -> &'a PyAny {
    if let Ok(py_bytes) = input.downcast_exact::<PyBytes>() {
        let json_bytes: &[u8] = py_bytes.as_bytes();
        let py_dict = jiter::python_parse(py, json_bytes, false, false)
            .map_err(|e| jiter::map_json_error(json_bytes, &e));
        py_dict.unwrap().into_ref(py)
    } else if let Ok(py_str) = input.downcast::<PyString>() {
        let cow_str = py_str.to_string_lossy();
        let json_bytes: &[u8] = cow_str.as_bytes();
        let py_dict = jiter::python_parse(py, json_bytes, false, false)
            .map_err(|e| jiter::map_json_error(json_bytes, &e));
        py_dict.unwrap().into_ref(py)
    } else if let Ok(py_dict) = input.downcast::<PyDict>() {
        py_dict
    } else {
        input
    }
}
#[pyfunction]
fn mysql(py: Python, value: &PyAny) -> PyResult<(String, PyObject)> {
    let py_value = format_input(py, value);
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

#[pyfunction]
fn mongo(py: Python, value: &PyAny) -> PyResult<PyObject> {
    let py_value = format_input(py, value);
    match py_value.downcast::<PyDict>() {
        Ok(py_dict) => match query::decode_python::decode_express(py_dict) {
            Ok(expr) => {
                return Ok(dialects::mongo::mongo(py, expr));
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
