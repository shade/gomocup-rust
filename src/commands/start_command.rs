use super::ExecutableCommand;

#[derive(Default)]
pub struct StartCommand;

impl ExecutableCommand for StartCommand {
    fn execute(&self, context: &mut super::game_context::GameContext, args: Vec<String>) -> Result<(), crate::errors::MooMooError> {
        println!("Start command executed");
        return Ok(());
    }
}