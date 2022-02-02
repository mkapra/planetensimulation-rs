mod field;
mod board;
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

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bla() {
        let mut board = Board::new(10, 5, 5, 5);
        board.generate_random_animals();
        println!("{}", board);
        board.step().unwrap();
        println!("{}", board);

        assert_eq!(true, true);
    }
}