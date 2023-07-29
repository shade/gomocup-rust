use std::{time::Duration, marker::PhantomData};

use crate::{Brain, game_board::GameBoard};

pub enum GameType {
    HumanOpponent,
    BrainOpponent,
    Tournament,
    NetworkTournament,
}

pub struct GameRules {
    pub is_exactly_five: bool,
    pub is_continuous_game: bool,
    pub is_renju: bool,
    pub is_caro: bool
}

pub struct GameContext<S: GameBoard, T: Brain> {
    pub brain: T,
    pub timeout_turn: Duration,
    pub timeout_match: Duration,
    pub max_memory: u64,
    pub time_left: Duration,
    pub game_type: GameType,
    pub rules: GameRules,
    pub phantom: PhantomData<S>
}

impl <S: GameBoard, T: Brain> GameContext<S,T> {
    pub fn new(brain: T) -> Self {
        return GameContext{ brain, timeout_turn: todo!(), timeout_match: todo!(), max_memory: todo!(), time_left: todo!(), game_type: todo!(), rules: todo!(), phantom: PhantomData::<S>{} };
    }
}