use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct MainFileNotFound;

impl fmt::Display for MainFileNotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "could not find 'niter.json'")
    }
}

impl Error for MainFileNotFound {}

#[derive(Debug)]
pub struct NotADirectory;

impl fmt::Display for NotADirectory {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "not a directory")
    }
}

impl Error for NotADirectory {}

#[derive(Debug)]
pub struct UnsupportedFormat(String);

impl fmt::Display for UnsupportedFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "format '{}' is not supported", self.0)
    }
}

impl Error for UnsupportedFormat {}