use crate::brain::{Brain, GameBoard};

pub struct RandomBrain;

impl RandomBrain {
    pub fn new() -> Self {
        return RandomBrain
    }
}

impl Brain for RandomBrain {
    fn pre_initialize(&self) {
        todo!()
    }

    fn make_move(&self, row: usize, col: usize) -> Result<(), crate::brain::BrainError> {
        todo!()
    }
}