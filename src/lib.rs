mod board;
mod field;
use std::fmt;

pub use board::Board;

/// Result type that is used by the library
pub type Result = std::result::Result<(), SimulationError>;

/// An error that can occur during the simulation
#[derive(Debug, PartialEq, Eq)]
pub struct SimulationError(String);

impl fmt::Display for SimulationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for SimulationError {}
