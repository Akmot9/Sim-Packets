use std::fmt;

#[derive(Debug)]
pub enum InterfaceError {
    NotFound(String),
}

impl fmt::Display for InterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterfaceError::NotFound(name) => write!(f, "Network interface '{}' not found", name),
        }
    }
}

impl std::error::Error for InterfaceError {}

use std::sync::PoisonError;

// Create a custom Error that we can return in Results
#[derive(Debug, thiserror::Error)]
pub enum Error {
    // Implement std::io::Error for our Error enum
    #[error(transparent)]
    Io(#[from] std::io::Error),
    // Add a PoisonError, but we implement it manually later
    #[error("the mutex was poisoned")]
    PoisonError(String),

    #[error("interface error: {0}")]
    InterfaceError(#[from] InterfaceError),
}
// Implement Serialize for the error
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
// Implement From<PoisonError> for Error to convert it to something we have set up serialization for
impl<T> From<PoisonError<T>> for Error {
    fn from(err: PoisonError<T>) -> Self {
        // We "just" convert the error to a string here
        Error::PoisonError(err.to_string())
    }
}
