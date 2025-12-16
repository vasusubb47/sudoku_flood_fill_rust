use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub struct Cell {
    value: Option<u8>,
    pub is_given: bool,
    candidates: u16,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            value: None,
            is_given: false,
            candidates: 0b11_1111_1110, // All candidates (1-9) are possible
        }
    }
}

impl Cell {
    pub fn set_value(&mut self, val: u8, is_given: bool) {
        self.value = Some(val);
        self.is_given = is_given;
        self.candidates = 0b0; // Reset to all candidates
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    pub fn get_number_of_candidates(&self) -> u8 {
        self.candidates.count_ones() as u8
    }

    pub fn get_value(&self) -> Option<u8> {
        self.value
    }

    pub fn get_possible_values(&self) -> Vec<u8> {
        let mut possible_values = Vec::new();
        for val in 1..10 {
            if (self.candidates & (1 << val)) != 0 {
                possible_values.push(val as u8);
            }
        }
        possible_values
    }

    pub fn print_info(&self) {
        match self.value {
            Some(v) => println!("Cell value: {}, is_given: {}", v, self.is_given),
            None => println!("Cell is empty, candidates: {:09b}", self.candidates),
        }
    }

    pub fn can_set_value(&self, val: u8) -> bool {
        // print debug info
        // println!("Checking if can set value {}, {:09b}: candidates = {:09b}", val, val, self.candidates);
        (self.candidates & (1 << val)) != 0
    }

    pub fn _set_candidate(&mut self, val: u8) {
        self.candidates |= 1 << val;
    }

    pub fn clear_candidate(&mut self, val: u8) {
        self.candidates &= !(1 << val);
        // print debug info
        // println!("Cleared candidate {}: now candidates = {:09b}", val, self.candidates);
    }

    pub fn _set_candidate_val (&mut self, candidates: u16) {
        self.candidates = candidates;
    }

    pub fn is_single_candidate(&self) -> Option<u8> {
        if self.has_value() {
            return None;
        }
        if self.candidates.count_ones() == 1 {
            let val = self.get_possible_values();
            return Some(val[0]);
        }
        None
    }
}
