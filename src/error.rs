use std::{
	alloc::LayoutError,
	fmt,
};

#[derive(Debug, PartialEq)]
pub enum Error {
	Address,
	Alloc,
	Layout(LayoutError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(formatter, "{}", match self {
			Error::Address => String::from("address not valid"),
			Error::Alloc => String::from("can\'t allocate"),
			Error::Layout(layout) => format!("{}", layout),
		})
	}
}

impl std::error::Error for Error {}


