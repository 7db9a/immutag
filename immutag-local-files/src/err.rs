pub use toml_edit::TomlError;

#[derive(Debug)]
pub enum ImmutagFileError {
    IoError(std::io::Error),
    TomlError(TomlError),
    BoxImmutagFileError(std::boxed::Box<ImmutagFileError>),
    Error(Error),
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    InvalidKey,
    InvalidFile,
    DuplicateKey,
    NoFile,
}

#[derive(Debug)]
pub struct Error {
    pub details: String,
    pub kind: ErrorKind,
}

impl Error {
    pub fn new(msg: &str, kind: ErrorKind) -> Error {
        Error {
            details: msg.to_string(),
            kind,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl From<std::boxed::Box<ImmutagFileError>> for ImmutagFileError {
    fn from(error: std::boxed::Box<ImmutagFileError>) -> Self {
        ImmutagFileError::BoxImmutagFileError(error)
    }
}

impl From<std::io::Error> for ImmutagFileError {
    fn from(error: std::io::Error) -> Self {
        ImmutagFileError::IoError(error)
    }
}

impl From<TomlError> for ImmutagFileError {
    fn from(error: TomlError) -> Self {
        ImmutagFileError::TomlError(error)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<Error> for ImmutagFileError {
    fn from(error: Error) -> Self {
        ImmutagFileError::Error(error)
    }
}
