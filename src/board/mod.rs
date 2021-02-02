mod r#impl;
mod default;
mod display;
mod fromstr;
mod internals;

use crate::*;

#[derive(Debug, Copy, Clone)]
pub struct Board {
    me: u64,
    op: u64,
    side: Side,
    moves: BoardMove
}

#[derive(Debug, Copy, Clone)]
pub enum ParseBoardError {
    InvalidMove(BoardMove),
    InvalidFormat
}
