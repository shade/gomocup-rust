use crate::Brain;

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

pub struct GameContext<S, T: Brain<S>> {
    pub brain: T,
    pub timeout_turn: Duration,
    pub timeout_match: Duration,
    pub max_memory: u64,
    pub time_left: Duration,
    pub game_type: GameType,
    pub rules: GameRules
}

impl GameContext {
    pub fn new<S, T: Brain<S>>(brain: T) -> Self {
        return GameContext{ brain, args: vec![] };
    }
}