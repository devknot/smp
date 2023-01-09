use std::{error, fmt, result};

#[derive(Debug, PartialEq)]
pub enum Error {
	Unknown,
	PoisonError,
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(formatter, "{}", match self {
			Error::Unknown => "unknown error",
			Error::PoisonError => "poison error", 
		})
	}
}

impl error::Error for Error {}

