pub use gomocup_rust::{Brain, run};

pub fn main() -> Result<(), gomocup_rust::GomocupError> {
    run(brain::RandomBrain::new())
}