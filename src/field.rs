//! This module contains the field struct and the field types
use colored::Colorize;
use log::{debug, info};
use rand::Rng;
use std::fmt;

type Position = (u32, u32);

const MAX_SHARK_LIFETIME: u32 = 8;
const SHARK_BREED_TIME: u32 = 8;
const FISH_BREED_TIME: u32 = 3;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnimalStatus {
    life: Option<u32>,
    breed_counter: u32,
}

impl AnimalStatus {
    fn new_fish() -> Self {
        AnimalStatus {
            life: None,
            breed_counter: FISH_BREED_TIME,
        }
    }

    fn new_shark() -> Self {
        let mut rng = rand::thread_rng();
        AnimalStatus {
            life: Some(rng.gen_range(1..SHARK_BREED_TIME)),
            breed_counter: SHARK_BREED_TIME,
        }
    }

    fn reduce_breet(&mut self) {
        self.breed_counter -= 1;
    }

    fn reduce_life(&mut self) {
        if let Some(life) = self.life {
            self.life = Some(life - 1);
        }
    }

    fn reset_life(&mut self) {
        if let Some(_) = self.life {
            self.life = Some(MAX_SHARK_LIFETIME)
        }
    }

    fn reset_breed(&mut self, r#type: &FieldType) {
        match r#type {
            FieldType::Fish => self.breed_counter = FISH_BREED_TIME,
            FieldType::Shark => self.breed_counter = SHARK_BREED_TIME,
            _ => (),
        }
    }

    fn is_dead(&self) -> bool {
        if let Some(life) = self.life {
            return life == 0;
        }

        false
    }

    pub fn has_to_breed(&self) -> bool {
        self.breed_counter == 0
    }
}

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
    pub status: Option<AnimalStatus>,
}

impl Field {
    /// Creates a new field
    ///
    /// # Arguments
    /// * `type` - The type of the field
    /// * `x` - The x coordinate of the field
    /// * `y` - The y coordinate of the field
    pub fn new(r#type: FieldType, x: u32, y: u32, status: Option<AnimalStatus>) -> Field {
        if let Some(status) = status {
            return Field {
                r#type,
                x,
                y,
                status: Some(status),
            };
        }

        match r#type {
            FieldType::Fish => Field {
                r#type,
                x,
                y,
                status: Some(AnimalStatus::new_fish()),
            },
            FieldType::Shark => Field {
                r#type,
                x,
                y,
                status: Some(AnimalStatus::new_shark()),
            },
            FieldType::Plankton => Field {
                r#type,
                x,
                y,
                status: None,
            },
        }
    }

    /// Removes a new position for its field
    ///
    /// # Arguments
    /// * `animals` - The board before the step
    ///
    /// # Returns
    /// The new position for the field
    pub fn step(&self, animals: &Vec<Vec<Field>>) -> Option<(Position, Option<AnimalStatus>)> {
        match self.r#type {
            FieldType::Fish => Some(self.get_next_fish_position(animals)),
            FieldType::Shark => self
                .get_next_shark_position(animals)
                .map(|((x, y), state)| ((x, y), Some(state))),
            _ => Some(((self.x, self.y), None)),
        }
    }

    fn get_next_fish_position(&self, animals: &[Vec<Field>]) -> (Position, Option<AnimalStatus>) {
        let mut new_status = self.status.clone().unwrap();
        if new_status.has_to_breed() {
            new_status.reset_breed(&self.r#type);
        }

        let mut possible_moves: Vec<Position> = vec![];

        let up = (
            self.x % (animals.first().unwrap().len() as u32),
            ((self.y + (animals.len() as u32) - 1) % (animals.len() as u32)),
        );
        if animals[up.1 as usize][up.0 as usize].check_field_empty() {
            possible_moves.push(up);
        }
        let down = (
            self.x % (animals.first().unwrap().len() as u32),
            ((self.y + (animals.len() as u32) + 1) % (animals.len() as u32)),
        );
        if animals[down.1 as usize][down.0 as usize].check_field_empty() {
            possible_moves.push(down);
        }
        let left = (
            ((self.x + (animals.first().unwrap().len() as u32) - 1) % (animals.len() as u32)),
            self.y % (animals.len() as u32),
        );
        if animals[left.1 as usize][left.0 as usize].check_field_empty() {
            possible_moves.push(left);
        }
        let right = (
            ((self.x + (animals.first().unwrap().len() as u32) + 1) % (animals.len() as u32)),
            self.y % (animals.len() as u32),
        );
        if animals[right.1 as usize][right.0 as usize].check_field_empty() {
            possible_moves.push(right);
        }

        new_status.reduce_breet();
        debug!(
            "Reduced breed counter for fish: {:?} old=({:?})",
            new_status, self.status
        );

        if possible_moves.is_empty() {
            return ((self.x, self.y), Some(new_status));
        }

        // Select random move
        let mut rng = rand::thread_rng();
        let move_index = rng.gen_range(0..possible_moves.len());
        info!(
            "Fish ({}, {}) moves to ({}, {})",
            self.x, self.y, possible_moves[move_index].0, possible_moves[move_index].1
        );
        (possible_moves[move_index], Some(new_status))
    }

    fn get_next_shark_position(&self, animals: &[Vec<Field>]) -> Option<(Position, AnimalStatus)> {
        let mut new_status = self.status.clone().unwrap();
        if new_status.has_to_breed() {
            new_status.reset_breed(&self.r#type);
        }
        new_status.reduce_breet();
        debug!(
            "Reduced breed counter for shark: {:?} old=({:?})",
            new_status.breed_counter, self.status
        );

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
        if animals[up.1 as usize][up.0 as usize].check_field_for_type(FieldType::Fish) {
            prioritized_moves.push(up);
        }
        if animals[down.1 as usize][down.0 as usize].check_field_for_type(FieldType::Fish) {
            prioritized_moves.push(down);
        }
        if animals[left.1 as usize][left.0 as usize].check_field_for_type(FieldType::Fish) {
            prioritized_moves.push(left);
        }
        if animals[right.1 as usize][right.0 as usize].check_field_for_type(FieldType::Fish) {
            prioritized_moves.push(right);
        }
        // Check for free fields around
        if animals[up.1 as usize][up.0 as usize].check_field_empty() {
            possible_moves.push(up);
        }
        if animals[down.1 as usize][down.0 as usize].check_field_empty() {
            possible_moves.push(down);
        }
        if animals[left.1 as usize][left.0 as usize].check_field_empty() {
            possible_moves.push(left);
        }
        if animals[right.1 as usize][right.0 as usize].check_field_empty() {
            possible_moves.push(right);
        }

        // If prioritized_moves is not empty then select a random move from it
        if !prioritized_moves.is_empty() {
            let index = rand::thread_rng().gen_range(0..prioritized_moves.len());
            new_status.reset_life();
            info!(
                "Shark ({}, {}) moves to prio field {:?}",
                self.x, self.y, prioritized_moves[index]
            );
            debug!(
                "New status for shark ({}, {}): {:?} old({:?})",
                self.x, self.y, new_status, self.status
            );
            return Some((prioritized_moves[index], new_status));
        }

        // Shark did not get any food so its life gets reduced
        new_status.reduce_life();
        if new_status.is_dead() {
            debug!("Shark ({}, {}) is dead", self.x, self.y);
            return None;
        }

        if possible_moves.is_empty() {
            return Some(((self.x, self.y), new_status));
        }

        // select a random move from possible_moves
        let index = rand::thread_rng().gen_range(0..possible_moves.len());
        info!(
            "Shark ({}, {}) moves to {:?}",
            self.x, self.y, possible_moves[index]
        );
        Some((possible_moves[index], new_status))
    }

    /// Check if the field has a specific type
    ///
    /// # Arguments
    /// * `type` - The type to check
    ///
    /// # Returns
    /// `true` if the field has the type, `false` otherwise
    fn check_field_for_type(&self, r#type: FieldType) -> bool {
        self.r#type == r#type
    }

    //// Check if the field is empty
    fn check_field_empty(&self) -> bool {
        self.check_field_for_type(FieldType::Plankton)
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.r#type {
            FieldType::Plankton => write!(f, "{}", "_".blue()),
            FieldType::Fish => write!(f, "{}", "F".green()),
            FieldType::Shark => write!(f, "{}", "S".red()),
        }
    }
}
