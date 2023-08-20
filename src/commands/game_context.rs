use std::{time::Duration, marker::PhantomData, default};

use crate::{Brain, GameBoard};

#[derive(Default)]
pub enum GameType {
    #[default]
    HumanOpponent,
    BrainOpponent,
    Tournament,
    NetworkTournament,
}

#[derive(Default)]
pub struct GameRules {
    pub is_exactly_five: bool,
    pub is_continuous_game: bool,
    pub is_renju: bool,
    pub is_caro: bool
}

#[derive(Default)]
pub struct GameContext {
    pub board: Option<Box<dyn GameBoard>>,
    pub timeout_turn: Duration,
    pub timeout_match: Duration,
    pub max_memory: u64,
    pub time_left: Duration,
    pub game_type: GameType,
    pub rules: GameRules,
}
