# gomocup-rust
(WIP) Rust Gomocup Interface


How to use this:

```
use gomocup_rust::{Brain, GomocupError, run}

fn main() -> Result<(), GomocupError> {
    // Construct your brain.
    let brain: Brain = CustomBrain::new();
    // Run the implementation.
    run(brain)
}

```