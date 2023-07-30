use super::{ExecutableCommand, CommandError, CommandResult};

#[derive(Default)]
pub struct StartCommand;

impl ExecutableCommand for StartCommand {
    fn execute(&self, context: &mut super::game_context::GameContext, args: Vec<String>) -> Result<CommandResult, CommandError> {
        println!("Start command executed");
        Ok(CommandResult::Nop)
    }
}