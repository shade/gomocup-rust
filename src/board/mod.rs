use std::default;

use crate::brain::GameConfig;


pub mod array;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoardError {
    InvalidPlace(String)
}


#[derive(Default)]
pub enum GamePiece {
    #[default]
    Empty,
    White,
    Black
}

impl std::ops::Not for GamePiece {
    type Output = GamePiece;

    fn not(self) -> Self::Output {
        match self {
            GamePiece::White => GamePiece::Black,
            GamePiece::Black => GamePiece::White,
            GamePiece::Empty => GamePiece::Empty
        }
    }
}

#[mockall::automock]
pub trait GameBoard: Default {
    fn new(size: u64) -> Result<Self, BoardError>;

    /// Get the length of the board, the board is always square.
    fn get_length(&self) -> usize {
        todo!();
    }

    /// Get the current piece to play.
    fn get_current_piece(&self) -> GamePiece {
        todo!();
    }

    fn place(&self, row: usize, col: usize) -> Result<(), BoardError> {
        todo!();
    }
    fn remove(&self, row: usize, col: usize) -> Result<(), BoardError> {
        todo!();
    }

    fn get_string(&self) -> String {
        let a: String = String::new();
        for row in 0..self.get_length() {
            for col in 0..self.get_length() {
            }
        }
        return "".to_string();
    }

    fn get_piece(&self, row: usize, col: usize) -> String {
        panic!("You must either implement display() or get_piece() for your GameBoard");
    }
}