use crate::brain::GameConfig;


pub mod array;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoardError {
    InvalidPlace(String)
}

pub enum GamePiece {
    White,
    Black
}

impl std::ops::Not for GamePiece {
    type Output = GamePiece;

    fn not(self) -> Self::Output {
        match self {
            GamePiece::White => GamePiece::Black,
            GamePiece::Black => GamePiece::White
        }
    }
}

#[mockall::automock]
pub trait GameBoard {
    fn get_n(&self) -> usize {
        todo!();
    }
    fn get_m(&self) -> usize {
        todo!();
    }

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
        for row in 0..self.get_n() {
            for col in 0..self.get_m() {
            }
        }
        return "".to_string();
    }

    fn get_piece(&self, row: usize, col: usize) -> String {
        panic!("You must either implement display() or get_piece() for your GameBoard");
    }
}