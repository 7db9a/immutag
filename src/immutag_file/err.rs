extern crate rustbreak;
extern crate toml_query;
pub use rustbreak::BreakError;
pub use toml_edit::TomlError;

#[derive(Debug)]
pub enum MetadataFileError {
    IoError(std::io::Error),
    TomlError(TomlError),
    BoxMetadataFileError(std::boxed::Box<MetadataFileError>),
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

impl From<std::boxed::Box<MetadataFileError>> for MetadataFileError {
    fn from(error: std::boxed::Box<MetadataFileError>) -> Self {
        MetadataFileError::BoxMetadataFileError(error)
    }
}

impl From<std::io::Error> for MetadataFileError {
    fn from(error: std::io::Error) -> Self {
        MetadataFileError::IoError(error)
    }
}

impl From<TomlError> for MetadataFileError {
    fn from(error: TomlError) -> Self {
        MetadataFileError::TomlError(error)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<Error> for MetadataFileError {
    fn from(error: Error) -> Self {
        MetadataFileError::Error(error)
    }
}
