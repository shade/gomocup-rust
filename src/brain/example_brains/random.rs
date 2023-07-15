use crate::brain::{Brain, GameBoard};

pub struct RandomBrain;

impl <T: GameBoard> Brain<T> for RandomBrain {
    fn pre_initialize() {
        // No-op
    }

    fn start() -> Self {
        self.game
    }

    fn make_move(row: usize, col: usize) -> Result<(), crate::brain::BrainError> {
        todo!()
    }

    fn get_board() -> T {
        todo!()
    }
}