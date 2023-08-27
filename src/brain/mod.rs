use mockall::automock;

use crate::{board::GameBoard, config::GameConfig};
pub mod random;
pub use random::RandomBrain;

#[derive(Debug)]
pub enum BrainError {
    /// Provides a string context for debugging purposes
    IllegalMove(String),
    CommonError(String),
    NoMoveFound,
}

#[automock]
pub trait Brain {
    /// Run before a game starts to do any initialization logic
    /// e.g. precomputing results, memory allocation, etc.
    fn pre_initialize(&mut self) {}

    /// Compute the next move on the gameboard. Depending on
    fn next_move<T: GameBoard + 'static>(
        &mut self,
        board: &mut T,
    ) -> Result<(u64, u64), BrainError>;

    /// Setup the game config for the brain.
    /// Though most of the GameConfig is sent at the beginning of the game it is possible
    /// for this value to change mid-game. The brain should be able to handle this.
    fn set_config(&mut self, config: GameConfig) -> Result<(), BrainError>;

    /// Run after a game ends to do any cleanup logic.
    fn end(&mut self) {}
}
