pub use pyo3::PyErr;
pub use pyo3::{create_exception, exceptions};
pub use std::error::Error;

create_exception!(jsql, JSQLError, exceptions::PyException);

pub fn py_error<T>(err: T) -> PyErr
where
    T: Error,
{
    JSQLError::new_err(err.to_string())
}
