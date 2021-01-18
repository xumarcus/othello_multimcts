mod r#impl;

mod seq;
mod par;

use crate::Board;

pub struct Node {
    board: Board,
    nodes: Vec<Box<Node>>,
    moves: Option<NonZeroU64>,
    m: Option<NonZeroU64>,
    w: usize,
    n: usize,
}
