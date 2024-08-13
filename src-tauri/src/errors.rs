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
