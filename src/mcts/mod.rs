mod r#impl;
mod internals;

use crate::Board;

use std::num::NonZeroU64;

pub struct Node {
    board: Board,
    moves: Option<NonZeroU64>,
    mmask: Option<NonZeroU64>,
    nwins: usize,
    visit: usize,
    nodes: Vec<Box<Node>>,
}

#[derive(PartialEq)]
pub struct NextMove {
    pub board: Board,
    pub mmask: NonZeroU64,
    pub score: f64
}

pub use r#impl::{mcts_seq, mcts_par};
