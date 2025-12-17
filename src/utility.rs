
pub enum Directional {
    Row,
    Column,
}

pub struct DirectionalCandidate {
    pub direction: Directional,
    pub index: u8,
    pub grid_row: usize,
    pub grid_col: usize,
    pub value: u8,
}

pub struct SingleCandidate {
    pub row: usize,
    pub column: usize,
    pub value: u8,
}

pub struct CellPossibleValues {
    pub row: usize,
    pub column: usize,
    pub value: Vec<u8>,
}

#[derive(Copy, Clone, Default)]
pub struct PrintCell{
    pub value: u8,
    pub number_of_candidates: u8,
    pub is_given: bool,
}