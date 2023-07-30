use crate::brain::GameConfig;


pub mod empty;

pub trait GameBoard {
    fn get_string(&self, config: &GameConfig) -> String {
        let a: String = String::new();
        for row in 0..config.game_n {
            for col in 0..config.game_m {
            }
        }
        return "".to_string();
    }

    fn get_piece(&self, row: usize, col: usize) {
        panic!("You must either implement display() or get_piece() for your GameBoard");
    }
}