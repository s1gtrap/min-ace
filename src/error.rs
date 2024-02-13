use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    Parse(mullvm_parser::Error),
}

impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Parse(e) => write!(f, "parse: {e}"),
        }
    }
}

impl From<mullvm_parser::Error> for Error {
    fn from(other: mullvm_parser::Error) -> Self {
        Error::Parse(other)
    }
}

impl Error {
    pub fn annotations(&self) -> Vec<crate::editor::Annotation> {
        vec![]
    }

    pub fn markers(&self) -> Vec<crate::editor::Marker> {
        vec![]
    }
}
