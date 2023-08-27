use std::{default, marker::PhantomData, time::Duration};

use crate::{config::GameConfig, Brain, GameBoard};

#[derive(Default)]
pub struct GameContext<T: GameBoard> {
    pub board: Option<T>,
    pub config: GameConfig,
}
