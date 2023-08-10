use crate::brain::GameConfig;


pub mod empty;
pub mod array;

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

pub trait GameBoard {
    fn get_n(&self) -> usize;
    fn get_m(&self) -> usize;

    fn get_current_piece(&self) -> GamePiece;

    fn place(&self, row: usize, col: usize) -> Result<(), BoardError>;
    fn remove(&self, row: usize, col: usize) -> Result<(), BoardError>;

    fn get_string(&self) -> String {
        let a: String = String::new();
        for row in 0..self.get_n() {
            for col in 0..self.get_m() {
                a.push_str(&self.get_piece(row, col));
            }
        }
        return "".to_string();
    }

    fn get_piece(&self, row: usize, col: usize) -> String {
        panic!("You must either implement display() or get_piece() for your GameBoard");
    }
}