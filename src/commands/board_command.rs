use std::io::BufRead;
use std::num::ParseIntError;

use crate::board::GamePiece;
use crate::{assert_argument_count, Brain, GameBoard};

use super::{CommandResult, ExecutableCommand, ExecutableCommandWithInput};
use crate::commands::game_context::GameContext;
use crate::commands::CommandError;

#[derive(Default)]
pub struct BoardCommand;

impl ExecutableCommandWithInput for BoardCommand {
    fn execute<G: GameBoard, B: Brain, R: BufRead>(
        &self,
        input: &mut R,
        brain: &mut B,
        context: &mut GameContext<G>,
        args: Vec<String>,
    ) -> Result<CommandResult, CommandError> {
        assert_argument_count!(args, 0);
        loop {
            let mut buffer = String::new();
            input.read_line(&mut buffer);

            if buffer.as_str() == "DONE" {
                break;
            }

            let args = buffer
                .split(",")
                .map(|s| s.parse::<u64>())
                .collect::<Result<Vec<u64>, ParseIntError>>()
                .map_err(|e| {
                    CommandError::InvalidArguments(format!("Invalid coordinates: {}", e))
                })?;

            if args.len() != 3 {
                return Err(CommandError::InvalidArguments(format!(
                    "Expected 3 comma seperated values, got {:?}",
                    args
                )));
            }

            let (row, col, piece_type) = (args[0], args[1], args[2]);
            println!(
                "placing piece at {}, {} {:?}",
                row,
                col,
                Into::<GamePiece>::into(piece_type)
            );
            if let Some(board) = context.board.as_mut() {
                board.place(row, col, piece_type.into())?;
            } else {
                return Err(CommandError::IllegalState(
                    "Board not initialized".to_string(),
                ));
            }
        }

        return Ok(CommandResult::Ok);
    }
}

#[cfg(test)]
mod test {
    use mockall::predicate::eq;

    use crate::{
        board::{GamePiece, MockGameBoard},
        brain::MockBrain,
        commands::{
            game_context::GameContext, CommandError, CommandResult, ExecutableCommandWithInput,
        },
    };

    use super::BoardCommand;
    use mockall::mock;

    use std::io::{BufRead, BufReader, Read};

    mock! {
        Reader{}
        impl Read for Reader {
            fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;
        }
        impl BufRead for Reader {
            fn fill_buf(&mut self) -> std::io::Result<&'static [u8]>;

            fn consume(&mut self, amt: usize);

            fn read_line(&mut self, buf: &mut String) -> std::io::Result<usize>;
        }
    }

    #[test]
    fn places_pieces_on_board() {
        let mut brain = MockBrain::default();
        let mut mock_board = MockGameBoard::default();
        let mut context = GameContext::<MockGameBoard>::default();

        let mut mock_reader = MockReader::default();

        mock_board
            .expect_place()
            .with(eq(0), eq(1), eq(GamePiece::White))
            .times(1)
            .return_const(Ok(()));
        mock_board
            .expect_place()
            .with(eq(1), eq(3), eq(GamePiece::Black))
            .times(1)
            .return_const(Ok(()));

        mock_reader.expect_read_line().times(1).return_once(|buf| {
            buf.push_str("0,1,2");
            Ok(5)
        });
        mock_reader.expect_read_line().times(1).returning(|buf| {
            buf.push_str("1,3,1");
            Ok(5)
        });
        mock_reader.expect_read_line().times(1).return_once(|buf| {
            buf.push_str("DONE");
            Ok(4)
        });

        context.board = Some(mock_board);
        let result = BoardCommand::default()
            .execute(&mut mock_reader, &mut brain, &mut context, vec![])
            .unwrap();

        assert_matches::assert_matches!(result, CommandResult::Ok);
    }
}
