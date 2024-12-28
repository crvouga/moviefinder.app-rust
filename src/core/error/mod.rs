#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Error {
    message: String,
}

impl Error {
    /// Creates a new `Error` with a custom message.
    pub fn new<I: Into<String>>(message: I) -> Self {
        Error {
            message: message.into(),
        }
    }

    pub fn namespace(&self, namespace: &str) -> Self {
        Error {
            message: format!("{}: {}", namespace, self.message),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

// Implement From for common error types
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::new(format!("IO error: {}", err))
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::new(format!("ParseInt error: {}", err))
    }
}

// Add more `From` implementations as needed

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::new(err)
    }
}
