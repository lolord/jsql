use std::fmt;

#[derive(Debug, Clone)]
pub enum ExpressError {
    ValidateError(String),
    OperatorError(String),
    Message(String),
    ValueError,
}

impl fmt::Display for ExpressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message())
    }
}

impl std::error::Error for ExpressError {}

impl ExpressError {
    fn message(&self) -> &str {
        match self {
            ExpressError::ValidateError(msg) => msg.as_str(),
            ExpressError::OperatorError(msg) => msg.as_str(),
            ExpressError::ValueError => "ValueError",
            ExpressError::Message(msg) => msg,
        }
    }
}
