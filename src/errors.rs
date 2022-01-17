// error1.rs
use crate::Location;
use std::convert::From;
use std::error::Error;
use std::fmt;

pub type GenericError = Box<dyn Error + Send + Sync>;
pub type GenericResult<T> = Result<T, GenericError>;

#[derive(Debug)]
pub struct LocationError {
  details: String,
}

impl From<&Location> for LocationError {
  fn from(location: &Location) -> Self {
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

#[derive(Debug)]
pub struct MoveError {
  details: String,
}

impl MoveError {
  pub fn new(message: &str) -> MoveError {
    MoveError {
      details: message.to_string(),
    }
  }
}
impl fmt::Display for MoveError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.details)
  }
}

impl Error for MoveError {
  fn description(&self) -> &str {
    &self.details
  }
}

#[derive(Debug)]
pub struct WrongPlayer {
  details: String,
}

impl WrongPlayer {
  pub fn new(message: &str) -> WrongPlayer {
    WrongPlayer {
      details: message.to_string(),
    }
  }
}
impl fmt::Display for WrongPlayer {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.details)
  }
}

impl Error for WrongPlayer {
  fn description(&self) -> &str {
    &self.details
  }
}

#[derive(Debug)]
pub struct TurnError {
  details: String,
}

impl TurnError {
  pub fn new(message: &str) -> TurnError {
    TurnError {
      details: message.to_string(),
    }
  }
}
impl fmt::Display for TurnError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.details)
  }
}

impl Error for TurnError {
  fn description(&self) -> &str {
    &self.details
  }
}
