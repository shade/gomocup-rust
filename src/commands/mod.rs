
use strum::IntoStaticStr;

use crate::{context::Context, brain::Brain};
pub mod context;

#[derive(IntoStaticStr, Display, EnumString)]
pub enum Command {
    #[strum(serialize="START")]
    Start,
    #[strum(serialize="TURN")]
    Turn,
    #[strum(serialize="BEGIN")]
    Begin,
    #[strum(serialize="BOARD")]
    Board,
    #[strum(serialize="INFO")]
    Info,
    #[strum(serialize="END")]
    End,
}

impl Command {
    pub fn execute<S, T:Brain<S>>(&self, context: &mut Context<S,T>) {
        self.into();
        match self {
            Command::Start => start(context),
            Command::Turn => turn(context),
            Command::Begin => begin(context),
            Command::Board => board(context),
            Command::Info => info(context),
            Command::End => end(context),
        }
    }
}

fn start<S, T:Brain<S>>(context: &mut Context<S,T>) {
    context.get_args();
    
}

fn turn<S, T:Brain<S>>(context: &mut Context<S,T>) {
    context.set_brain(T::new());   
}

fn begin<S, T:Brain<S>>(context: &mut Context<S,T>) {
    context.set_brain(T::new());   
}


fn board<S, T:Brain<S>>(context: &mut Context<S,T>) {
    context.set_brain(T::new());   
}


fn info<S, T:Brain<S>>(context: &mut Context<S,T>) {
    context.set_brain(T::new());   
}

fn end<S, T:Brain<S>>(context: &mut Context<S,T>) {
    context.set_brain(T::new());   
}
