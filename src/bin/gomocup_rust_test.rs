pub use gomocup_rust::{run, Brain};
use gomocup_rust::ArrayBoard;
use gomocup_rust::brain::RandomBrain;

pub fn main() -> Result<(), gomocup_rust::GomocupError> {
    run::<_, ArrayBoard>(&mut RandomBrain::default())
}
