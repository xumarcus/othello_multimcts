mod r#impl;
mod from_env;

use super::*;

#[derive(Debug, Default)]
pub struct Game {
    side: Side,
    board: Board,
    black: Boxed,
    white: Boxed,
    pub verbose: usize
}
