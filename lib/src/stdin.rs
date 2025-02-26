use std::io::{self, IsTerminal};

pub trait Terminal {
    fn is_terminal(&self) -> bool;
}

pub struct StdinService;
impl StdinService {
    pub fn new() -> Self {
        Self {}
    }
}
impl Terminal for StdinService {
    fn is_terminal(&self) -> bool {
        io::stdin().is_terminal()
    }
}