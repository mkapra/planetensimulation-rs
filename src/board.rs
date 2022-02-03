//! Contains the board struct. A board contains all the necessary information and controls the simulation.
//!
//! # Examples
//! ```
//! use planetensimulation::Board;
//!
//! // Board with 10 fishes and 5 sharks with 25x25 fields
//! let mut board = Board::new(10, 5, 25, 25);
//! board.generate_random_animals();
//! //board.draw();
//! ```
use log::debug;
use rand::Rng;

use crate::{
    field::{Field, FieldType},
    Result, SimulationError,
};

/// Holds all the fields and information of the simulation
#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    amount_fishes: u32,
    amount_sharks: u32,
    rows: u32,
    columns: u32,
    fields: Vec<Vec<Field>>,
}

impl Board {
    /// Creates a new board
    ///
    /// # Arguments
    /// * `rows` - The amount of rows
    /// * `columns` - The amount of columns
    /// * `amount_fishes` - The amount of fishes that are placed on the board initially
    /// * `amount_sharks` - The amount of sharks that are placed on the board initially
    ///
    /// # Panics
    /// If the amount of fishes and sharks is greater than the amount of fields
    pub fn new(amount_fishes: u32, amount_sharks: u32, rows: u32, columns: u32) -> Self {
        // If the amount of fishes and sharks is bigger than the amount of fields panic
        if amount_fishes + amount_sharks > rows * columns {
            panic!("The amount of fishes and sharks is bigger than the amount of fields");
        }

        Board {
            amount_fishes,
            amount_sharks,
            rows,
            columns,
            fields: Vec::with_capacity((amount_fishes + amount_sharks) as usize),
        }
    }

    /// Generate a new board with the amount of fishes and sharks
    pub fn generate_random_animals(&mut self) {
        let mut rand_gen = rand::thread_rng();

        // Initialize an empty 2d vector
        let mut animals: Vec<Vec<Field>> = Vec::with_capacity(self.rows as usize);
        for y in 0..self.rows {
            let mut animals_row = Vec::with_capacity(self.columns as usize);
            for x in 0..self.columns {
                animals_row.push(Field::new(FieldType::Plankton, x, y, None));
            }
            animals.push(animals_row)
        }

        // Randomly insert fishes into the empty field
        for _ in 0..self.amount_fishes {
            let mut random_x = rand_gen.gen_range(0..animals.first().unwrap().len());
            let mut random_y = rand_gen.gen_range(0..animals.len());

            while animals[random_y][random_x].r#type != FieldType::Plankton {
                random_x = rand_gen.gen_range(0..animals.first().unwrap().len());
                random_y = rand_gen.gen_range(0..animals.len());
            }
            animals[random_y][random_x] =
                Field::new(FieldType::Fish, random_x as u32, random_y as u32, None);
        }
        // Randomly insert sharks into the empty field
        for _ in 0..self.amount_sharks {
            let mut random_col = rand_gen.gen_range(0..animals.first().unwrap().len());
            let mut random_row = rand_gen.gen_range(0..animals.len());

            while animals[random_row][random_col].r#type != FieldType::Plankton {
                random_col = rand_gen.gen_range(0..animals.first().unwrap().len());
                random_row = rand_gen.gen_range(0..animals.len());
            }

            animals[random_row][random_col] =
                Field::new(FieldType::Shark, random_col as u32, random_row as u32, None);
        }

        self.fields = animals;
        debug!("Initial state:\n{}", self);
    }

    /// Simulates one step of the simulation
    ///
    /// # Errors
    /// If there are no sharks or fishes on the board
    pub fn step(&mut self) -> Result {
        let cloned_fields = self.fields.clone();
        let fishes = Self::get_fishes(&cloned_fields);
        let sharks = Self::get_sharks(&cloned_fields);

        if fishes.len() == 0 || sharks.len() == 0 {
            return Err(SimulationError(
                "No fishes or sharks left on the board".into(),
            ));
        }

        for fish in fishes {
            let (old_x, old_y) = (fish.x, fish.y);
            let ((new_x, new_y), status) = fish.step(&self.fields).unwrap();

            if status.as_ref().unwrap().has_to_breed() {
                self.fields[old_y as usize][old_x as usize] =
                    Field::new(FieldType::Fish, old_x, old_y, None);
            } else {
                // Set old field to plankton
                self.fields[old_y as usize][old_x as usize] =
                    Field::new(FieldType::Plankton, old_x, old_y, None);
            }
            // Set new field to fish
            self.fields[new_y as usize][new_x as usize] =
                Field::new(FieldType::Fish, new_x, new_y, status);
        }

        debug!("After fish moves:\n{}", self);

        for shark in sharks {
            let (old_x, old_y) = (shark.x, shark.y);
            if let Some(((new_x, new_y), status)) = shark.step(&self.fields) {
                if status.as_ref().unwrap().has_to_breed() {
                    self.fields[old_y as usize][old_x as usize] =
                        Field::new(FieldType::Shark, old_x, old_y, None);
                } else {
                    // Set old field to plankton
                    self.fields[old_y as usize][old_x as usize] =
                        Field::new(FieldType::Plankton, old_x, old_y, None);
                }

                // Set new field to shark
                self.fields[new_y as usize][new_x as usize] =
                    Field::new(FieldType::Shark, new_x, new_y, status);
            } else {
                // Set old field to plankton
                self.fields[old_y as usize][old_x as usize] =
                    Field::new(FieldType::Plankton, old_x, old_y, None);
            }
        }

        Ok(())
    }

    fn get_fishes(animals: &[Vec<Field>]) -> Vec<&Field> {
        animals
            .iter()
            .flatten()
            .filter(|field| field.r#type == FieldType::Fish)
            .collect()
    }

    fn get_sharks(animals: &[Vec<Field>]) -> Vec<&Field> {
        animals
            .iter()
            .flatten()
            .filter(|field| field.r#type == FieldType::Shark)
            .collect()
    }

    /// Count the animals that are currently on the board
    pub fn count_animals(&self) -> (u32, u32) {
        let mut fishes = 0;
        let mut sharks = 0;
        for row in self.fields.iter() {
            for field in row.iter() {
                match field.r#type {
                    FieldType::Fish => fishes += 1,
                    FieldType::Shark => sharks += 1,
                    _ => (),
                }
            }
        }
        (fishes, sharks)
    }
}

use std::fmt::{Display, Formatter, Result as FmtResult};
impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        for row in self.fields.iter() {
            for field in row {
                write!(f, "{}, ", field)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
