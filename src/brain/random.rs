use crate::{board::GamePiece, config::GameConfig, Brain, BrainError, GameBoard};

#[derive(Default)]
pub struct RandomBrain;

impl Brain for RandomBrain {
    fn next_move<T: GameBoard + 'static>(
        &mut self,
        board: &mut T,
    ) -> Result<(u64, u64), BrainError> {
        let mut i = 0;
        loop {
            // Prevent infinite loops,
            // on a 19x19 board, probability we don't hit 1 available square
            // after 100k tries is roughly 10^-121, so we're fine.
            if i >= 100_000 {
                return Err(BrainError::NoMoveFound);
            }

            i += 1;

            let row = rand::random::<u64>() % board.get_length();
            let col = rand::random::<u64>() % board.get_length();
            if matches!(board.get_piece(row, col), GamePiece::Empty) {
                return Ok((row, col));
            }
        }
    }

    fn set_config(&mut self, config: GameConfig) -> Result<(), BrainError> {
        Ok(())
    }
}
