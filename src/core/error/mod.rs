#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct CoreError {
    message: String,
}

impl CoreError {
    /// Creates a new `Error` with a custom message.
    pub fn new<I: Into<String>>(message: I) -> Self {
        CoreError {
            message: message.into(),
        }
    }

    pub fn namespace(&self, namespace: &str) -> Self {
        CoreError {
            message: format!("{}: {}", namespace, self.message),
        }
    }
}

impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CoreError {}

// Implement From for common error types
impl From<std::io::Error> for CoreError {
    fn from(err: std::io::Error) -> Self {
        CoreError::new(format!("IO error: {}", err))
    }
}

impl From<std::num::ParseIntError> for CoreError {
    fn from(err: std::num::ParseIntError) -> Self {
        CoreError::new(format!("ParseInt error: {}", err))
    }
}

// Add more `From` implementations as needed

impl From<String> for CoreError {
    fn from(err: String) -> Self {
        CoreError::new(err)
    }
}
