use std::{sync::Arc, time::Duration};

use strum::IntoEnumIterator;

use crate::commands::CommandError;

#[derive(PartialEq, Eq, Clone, Copy, Debug, strum::EnumIter)]
pub enum GameRule {
    ExactlyFive,
    ContinuousGame,
    Renju,
    Caro,
}

impl Into<u64> for GameRule {
    fn into(self) -> u64 {
        match self {
            GameRule::ExactlyFive => 1,
            GameRule::ContinuousGame => 2,
            GameRule::Renju => 4,
            GameRule::Caro => 8,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct GameRules(Vec<GameRule>);

impl From<u64> for GameRules {
    fn from(rule_mask: u64) -> Self {
        let mut game_rules = Vec::new();

        for rule in GameRule::iter() {
            if rule_mask & Into::<u64>::into(rule) != 0 {
                game_rules.push(rule);
            }
        }

        GameRules(game_rules)
    }
}

#[derive(Debug, Default, Clone)]
pub struct GameConfig {
    pub game_n: usize,
    pub game_rule: GameRules,
    pub game_type: GameType,
    pub max_memory: u64,
    pub timeout_turn: Duration,
    pub timeout_match: Duration,
    pub time_left: Duration,
    pub persisted_files_folder: Arc<String>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum GameType {
    #[default]
    HumanOpponent,
    BrainOpponent,
    Tournament,
    NetworkTournament,
}

impl From<u64> for GameType {
    fn from(game_type: u64) -> Self {
        match game_type {
            0 => GameType::HumanOpponent,
            1 => GameType::BrainOpponent,
            2 => GameType::Tournament,
            3 => GameType::NetworkTournament,
            _ => GameType::HumanOpponent,
        }
    }
}
