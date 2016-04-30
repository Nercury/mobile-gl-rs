use egli::error::Error as EglError;
use std::result;

#[derive(Copy, Clone, Debug)]
pub enum Error {
    EGLError(EglError)
}

impl From<EglError> for Error {
    fn from(other: EglError) -> Self {
        Error::EGLError(other)
    }
}

pub type Result<T> = result::Result<T, Error>;
