use gomocup_rust::brain::RandomBrain;
use gomocup_rust::ArrayBoard;
pub use gomocup_rust::{run, Brain};

pub fn main() -> Result<(), gomocup_rust::GomocupError> {
    run::<_, ArrayBoard>(&mut RandomBrain::default())
}
