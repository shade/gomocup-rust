use crate::Brain;

pub struct Context<S, T: Brain<S>> {
    pub brain: T,
    pub args: Vec<String>
}

impl Context {
    pub fn new<S, T: Brain<S>>(brain: T) -> Self {
        return Context{ brain, args: vec![] };
    }

    pub fn set_args(&mut self, args: String) {
        self.args = args;
    }
}