use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub struct Cell {
    value: Option<u8>,
    is_given: bool,
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

    pub fn can_set_value(&self, val: u8) -> bool {
        // print debug info
        println!("Checking if can set value {}, {:09b}: candidates = {:09b}", val, val, self.candidates);
        (self.candidates & (1 << val)) != 0
    }

    pub fn _set_candidate(&mut self, val: u8) {
        self.candidates |= 1 << val;
    }

    pub fn clear_candidate(&mut self, val: u8) {
        self.candidates &= !(1 << val);
        // print debug info
        println!("Cleared candidate {}: now candidates = {:09b}", val, self.candidates);
    }

    pub fn _set_candidate_val (&mut self, candidates: u16) {
        self.candidates = candidates;
    }

    pub fn _is_single_candidate(&self) -> bool {
        self.candidates.count_ones() == 1
    }
}
