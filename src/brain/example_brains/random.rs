use crate::brain::{Brain, GameBoard, BrainError, GameConfig};

pub struct RandomBrain;

impl RandomBrain {
    pub fn new() -> Self {
        return RandomBrain
    }
}

impl Brain for RandomBrain {

    /// Get the next move on the board.
    fn next_move<T: GameBoard + 'static>(&self, board: T) -> Result<(), BrainError> {
        Ok(())
    }

    /// Setup the game config for the brain.
    fn set_config(config: GameConfig) -> Result<(), BrainError> {
        Ok(())
    }
}