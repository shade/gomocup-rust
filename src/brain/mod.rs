use mockall::automock;

use crate::game_board::GameBoard;

pub mod example_brains;


pub struct GameConfig {
    pub game_n: usize,
    pub game_m: usize,
    pub game_k: usize
}

pub enum BrainError {
    /// Provides a string context for debugging purposes
    IllegalMove(String),

    /// 
    CommonError(String)
}

#[automock]
pub trait Brain {
    /// Run before a game starts to do any initialization logic
    /// e.g. precomputing results, memory allocation, etc.
    fn pre_initialize(&self);

    /// Used to make a move, should return IllegalMove if the move
    /// is out of bounds.
    fn make_move(&self, row: usize, col: usize) -> Result<(), BrainError>;

}
