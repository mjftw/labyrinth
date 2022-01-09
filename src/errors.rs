// error1.rs
use crate::Location;
use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct LocationError {
  details: String,
}

impl From<Location> for LocationError {
  fn from(location: Location) -> Self {
    LocationError {
      details: format!("Invalid location {}", location),
    }
  }
}

impl LocationError {
  pub fn new(message: &str) -> Self {
    LocationError {
      details: message.to_string(),
    }
  }
}

impl fmt::Display for LocationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.details)
  }
}

impl Error for LocationError {
  fn description(&self) -> &str {
    &self.details
  }
}
