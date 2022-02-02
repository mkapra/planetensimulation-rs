//! This module contains the field struct and the field types
use log::debug;
use rand::Rng;

type Position = (u32, u32);

/// Represents a type of a field
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FieldType {
    Shark,
    Fish,
    Plankton,
}

/// Represents a field on the board
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub r#type: FieldType,
    pub x: u32,
    pub y: u32,
}

impl Field {
    /// Creates a new field
    ///
    /// # Arguments
    /// * `type` - The type of the field
    /// * `x` - The x coordinate of the field
    /// * `y` - The y coordinate of the field
    pub fn new(r#type: FieldType, x: u32, y: u32) -> Field {
        Field { r#type, x, y }
    }

    /// Removes a new position for its field
    ///
    /// # Arguments
    /// * `animals` - The board before the step
    ///
    /// # Returns
    /// The new position for the field
    pub fn step(&self, animals: &Vec<Vec<Field>>) -> Position {
        match self.r#type {
            FieldType::Fish => self.get_next_fish_position(animals),
            FieldType::Shark => self.get_next_shark_position(animals),
            _ => (self.x, self.y),
        }
    }

    fn get_next_fish_position(&self, animals: &[Vec<Field>]) -> Position {
        let mut possible_moves: Vec<Position> = vec![];

        let up = (
            self.x % (animals.first().unwrap().len() as u32),
            ((self.y + (animals.len() as u32) - 1) % (animals.len() as u32)),
        );
        if let FieldType::Plankton = animals[up.1 as usize][up.0 as usize].r#type {
            possible_moves.push(up);
        }
        let down = (
            self.x % (animals.first().unwrap().len() as u32),
            ((self.y + (animals.len() as u32) + 1) % (animals.len() as u32)),
        );
        if let FieldType::Plankton = animals[down.1 as usize][down.0 as usize].r#type {
            possible_moves.push(down);
        }
        let left = (
            ((self.x + (animals.first().unwrap().len() as u32) - 1) % (animals.len() as u32)),
            self.y % (animals.len() as u32),
        );
        if let FieldType::Plankton = animals[left.1 as usize][left.0 as usize].r#type {
            possible_moves.push(left);
        }
        let right = (
            ((self.x + (animals.first().unwrap().len() as u32) + 1) % (animals.len() as u32)),
            self.y % (animals.len() as u32),
        );
        if let FieldType::Plankton = animals[right.1 as usize][right.0 as usize].r#type {
            possible_moves.push(right);
        }

        if possible_moves.is_empty() {
            return (self.x, self.y);
        }

        // Select random move
        let mut rng = rand::thread_rng();
        let move_index = rng.gen_range(0..possible_moves.len());
        debug!(
            "Fish ({}, {}) moves to ({}, {})",
            self.x, self.y, possible_moves[move_index].0, possible_moves[move_index].1
        );
        possible_moves[move_index]
    }

    fn get_next_shark_position(&self, animals: &[Vec<Field>]) -> Position {
        let mut prioritized_moves: Vec<Position> = vec![];
        let mut possible_moves: Vec<Position> = vec![];

        let up = (
            self.x % (animals.first().unwrap().len() as u32),
            ((self.y + (animals.len() as u32) - 1) % (animals.len() as u32)),
        );
        let down = (
            self.x % (animals.first().unwrap().len() as u32),
            ((self.y + (animals.len() as u32) + 1) % (animals.len() as u32)),
        );
        let left = (
            ((self.x + (animals.first().unwrap().len() as u32) - 1) % (animals.len() as u32)),
            self.y % (animals.len() as u32),
        );
        let right = (
            ((self.x + (animals.first().unwrap().len() as u32) + 1) % (animals.len() as u32)),
            self.y % (animals.len() as u32),
        );

        // Check if there is a fish in the neighbour fields
        if let FieldType::Fish = animals[up.1 as usize][up.0 as usize].r#type {
            prioritized_moves.push(up);
        }
        if let FieldType::Fish = animals[down.1 as usize][down.0 as usize].r#type {
            prioritized_moves.push(down);
        }
        if let FieldType::Fish = animals[left.1 as usize][left.0 as usize].r#type {
            prioritized_moves.push(left);
        }
        if let FieldType::Fish = animals[right.1 as usize][right.0 as usize].r#type {
            prioritized_moves.push(right);
        }
        // Check for free fields around
        if let FieldType::Plankton = animals[up.1 as usize][up.0 as usize].r#type {
            possible_moves.push(up);
        }
        if let FieldType::Plankton = animals[down.1 as usize][down.0 as usize].r#type {
            possible_moves.push(down);
        }
        if let FieldType::Plankton = animals[left.1 as usize][left.0 as usize].r#type {
            possible_moves.push(left);
        }
        if let FieldType::Plankton = animals[right.1 as usize][right.0 as usize].r#type {
            possible_moves.push(right);
        }

        // If prioritized_moves is not empty then select a random move from it
        if !prioritized_moves.is_empty() {
            let index = rand::thread_rng().gen_range(0..prioritized_moves.len());
            debug!(
                "Shark ({}, {}) moves to prio field {:?}",
                self.x, self.y, prioritized_moves[index]
            );
            return prioritized_moves[index];
        }

        if possible_moves.is_empty() {
            return (self.x, self.y);
        }

        // select a random move from possible_moves
        let index = rand::thread_rng().gen_range(0..possible_moves.len());
        debug!(
            "Shark ({}, {}) moves to {:?}",
            self.x, self.y, possible_moves[index]
        );
        return possible_moves[index];
    }
}
