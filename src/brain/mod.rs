use mockall::automock;

use crate::board::GameBoard;

pub mod example_brains;


pub struct GameConfig {
    pub game_n: usize,
    pub game_m: usize,
    pub game_k: usize
}

pub enum BrainError {
    /// Provides a string context for debugging purposes
    IllegalMove(String),

    
    CommonError(String)
}

#[automock]
pub trait Brain {
    /// Run before a game starts to do any initialization logic
    /// e.g. precomputing results, memory allocation, etc.
    fn pre_initialize(&self);

    /// Get the next move on the board.
    fn next_move<T: GameBoard + 'static>(&self, board: T) -> Result<(), BrainError>;

    /// Setup the game config for the brain.
    fn set_config(config: GameConfig) -> Result<(), BrainError>;
}
