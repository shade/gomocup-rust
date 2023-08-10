use crate::GameBoard;

use super::{GamePiece, BoardError};

pub struct ArrayBoard {
    pub board: Vec<Vec<Option<GamePiece>>>,
    pub n: usize,
    pub m: usize,
    pub k: usize,
    pub current_piece: GamePiece
}

impl ArrayBoard {
    fn get_piece(&self, row: usize, col: usize) -> Result<&mut Option<GamePiece>, BoardError> {
        let game_piece = self.board.get(row)
            .ok_or(BoardError::InvalidPlace(format!("Row is out of bounds: {}", row)))?
            .get_mut(col)
            .ok_or(BoardError::InvalidPlace(format!("Column is out of bounds: {}", col)))?;
        return None;
    }
}


impl GameBoard for ArrayBoard {
    fn get_n(&self) -> usize {
        self.n
    }

    fn get_m(&self) -> usize {
        self.m
    }

    fn get_current_piece(&self) -> GamePiece {
        return GamePiece::White;
    }

    fn place(&mut self, row: usize, col: usize) -> Result<(), BoardError> {
        let game_piece = self.get_piece(row, col)?;

        if game_piece.is_some() {
            return Err(BoardError::InvalidPlace(format!("Cannot place piece at {}, {} because it is already occupied", row, col)));
        }

        game_piece.replace(self.game_piece);
        self.game_piece = !self.game_piece;
        return Ok(());
    }

    fn remove(&mut self, row: usize, col: usize) -> Result<(), BoardError> {
        let game_piece = self.get_piece(row, col)?;

        if game_piece.is_none() {
            return Err(BoardError::InvalidPlace(format!("Cannot remove piece at {}, {} because it is already empty", row, col)));
        }

        game_piece.take();
        self.game_piece = !self.game_piece;
        
        return Ok(());
    }
}
