use super::GameBoard;


pub struct EmptyBoard;

impl GameBoard for EmptyBoard {
    fn get_n(&self) -> usize {
        todo!()
    }

    fn get_m(&self) -> usize {
        todo!()
    }

    fn place(&self, row: usize, col: usize) {
        todo!()
    }

    fn remove(&self, row: usize, col: usize) {
        todo!()
    }
}