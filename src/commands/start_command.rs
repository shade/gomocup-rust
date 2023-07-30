use super::{ExecutableCommand, CommandError};

#[derive(Default)]
pub struct StartCommand;

impl ExecutableCommand for StartCommand {
    fn execute(&self, context: &mut super::game_context::GameContext, args: Vec<String>) -> Result<(), CommandError> {
        println!("Start command executed");
        return Ok(());
    }
}