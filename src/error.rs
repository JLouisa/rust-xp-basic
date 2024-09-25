use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    TaskError,
    Io(std::io::Error),
    Other(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{:?}", self)
    }
}

impl std::error::Error for Error {}
