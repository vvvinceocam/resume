use std::error::Error as StdError;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    BranchDoesntExist(String, git2::Error),
    Configuration(YamlErrorWrapper),
    Format(std::fmt::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IO(_) => write!(f, "I/O error"),
            Self::BranchDoesntExist(name, _) => write!(f, "Branch {} doesn't exist", name),
            Self::Configuration(_) => write!(f, "Invalid configuration"),
            Self::Format(_) => write!(f, "Formatting error"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::IO(source) => Some(source),
            Self::BranchDoesntExist(_, source) => Some(source),
            Self::Configuration(source) => Some(source),
            Self::Format(source) => Some(source),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(error: serde_yaml::Error) -> Self {
        Error::Configuration(YamlErrorWrapper(error))
    }
}

impl From<std::fmt::Error> for Error {
    fn from(error: std::fmt::Error) -> Self {
        Error::Format(error)
    }
}

#[derive(Debug)]
pub struct YamlErrorWrapper(serde_yaml::Error);

impl fmt::Display for YamlErrorWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl StdError for YamlErrorWrapper {}
