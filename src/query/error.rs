#[derive(Debug, Clone)]
pub enum ExpressError {
    ValidateError(String),
    OperatorError(String),
    Message(String),
    ValueError(String),
}

impl std::fmt::Display for ExpressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message())
    }
}

impl std::error::Error for ExpressError {}

impl ExpressError {
    fn message(&self) -> &str {
        match self {
            ExpressError::ValidateError(msg) => msg.as_str(),
            ExpressError::OperatorError(msg) => msg.as_str(),
            ExpressError::ValueError(msg) => msg.as_str(),
            ExpressError::Message(msg) => msg,
        }
    }
}
