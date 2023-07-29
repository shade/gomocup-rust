
use strum::IntoStaticStr;

use crate::{Brain, game_board::GameBoard};

use self::game_context::GameContext;
pub mod game_context;

pub mod input_options;

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
    pub fn execute<S: GameBoard, T: Brain>(&self, context: &mut GameContext<S,T>) {
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

fn start<S: GameBoard, T: Brain>(context: &mut GameContext<S,T>) {

}

fn turn<S: GameBoard, T: Brain>(context: &mut GameContext<S,T>) {

}

fn begin<S: GameBoard, T: Brain>(context: &mut GameContext<S,T>) {

}


fn board<S: GameBoard, T: Brain>(context: &mut GameContext<S,T>) {

}


fn info<S: GameBoard, T: Brain>(context: &mut GameContext<S,T>) {

}

fn end<S: GameBoard, T: Brain>(context: &mut GameContext<S,T>) {

}
