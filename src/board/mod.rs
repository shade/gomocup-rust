use crate::brain::GameConfig;


pub mod empty;
pub mod array;

pub enum GamePiece {
    White,
    Black
}

pub trait GameBoard {
    fn get_n(&self) -> usize;
    fn get_m(&self) -> usize;

    fn place(&self, row: usize, col: usize);
    fn remove(&self, row: usize, col: usize);

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