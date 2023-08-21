use crate::GameBoard;

use super::{GamePiece, BoardError};

#[derive(Default)]
pub struct ArrayBoard {
    pub board: Vec<Vec<Option<GamePiece>>>,
    pub n: usize,
    pub m: usize,
    pub k: usize,
    pub current_piece: GamePiece
}

impl ArrayBoard {
    fn get_piece<'a>(&'a mut self, row: usize, col: usize) -> Result<&'a mut Option<GamePiece>, BoardError> {
        let game_piece = self.board.get_mut(row)
            .ok_or(BoardError::InvalidPlace(format!("Row is out of bounds: {}", row)))?
            .get_mut(col)
            .ok_or(BoardError::InvalidPlace(format!("Column is out of bounds: {}", col)))?;
        Ok(game_piece)
    }
}


impl GameBoard for ArrayBoard {
    fn new(size: u64) -> Result<Self, BoardError> {
        return Ok(ArrayBoard::default());
    }
}
