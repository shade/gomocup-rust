
pub struct GameConfig {
    pub game_n: usize,
    pub game_m: usize,
    pub game_k: usize
}

pub enum BrainError {
    /// Provides a string context for debugging purposes
    IllegalMove(String),

    /// 
    CommonError(String)
}

pub trait Brain<T: GameBoard> {
    /// Run before a game starts to do any initialization logic
    /// e.g. precomputing results, memory allocation, etc.
    fn pre_initialize();

    /// Called when a new instance of the game starts
    /// returns a new instance of the brain.
    fn start() -> Self;

    /// Used to make a move, should return IllegalMove if the move
    /// is out of bounds.
    fn make_move(row: usize, col: usize) -> Result<(), BrainError>;

    /// Return a stringified representation of the board.
    fn get_board() -> T;
}

pub trait GameBoard {
    fn get_string(&self, config: &GameConfig) -> String {
        let a: String = String::new();
        for row in 0..config.game_n {
            for col in 0..config.game_m {
                a.
            }
        }
    }

    fn get_piece(&self, row: usize, col: usize) {
        panic!("You must either implement display() or get_piece() for your GameBoard");
    }
}