use std::default;

use crate::brain::GameConfig;


pub mod array;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoardError {
    InvalidPlace(String)
}


#[derive(Default, Debug, Eq, PartialEq)]
pub enum GamePiece {
    #[default]
    Empty,
    White,
    Black
}

impl From<u64> for GamePiece {
    fn from(value: u64) -> Self {
        match value {
            1 => GamePiece::Black,
            2 => GamePiece::White,
            _ => GamePiece::Empty,
        }
    }
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
pub trait GameBoard : Default {
    fn new(size: u64) -> Result<Self, BoardError> where Self: Sized;

    /// Get the length of the board, the board is always square.
    fn get_length(&self) -> u64 {
        todo!();
    }

    /// Get the current piece to play.
    fn get_current_piece(&self) -> GamePiece {
        todo!();
    }

    fn place(&self, row: u64, col: u64, piece_type: GamePiece) -> Result<(), BoardError> {
        todo!();
    }
    fn remove(&self, row: u64, col: u64) -> Result<(), BoardError> {
        todo!();
    }

    fn get_piece(&self, row: u64, col: u64) -> GamePiece {
        panic!("You must either implement display() or get_piece() for your GameBoard");
    }
}