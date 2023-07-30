use crate::errors::MooMooError;

use super::{ExecutableCommand, game_context::GameContext, CommandError};

#[derive(Default)]
pub struct TurnCommand;

impl ExecutableCommand for TurnCommand {
    fn execute(&self, context: &mut GameContext, args: Vec<String>) -> Result<(), CommandError> {
        println!("Turn command executed");
        return Ok(());
    }
}
