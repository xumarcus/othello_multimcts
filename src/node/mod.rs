mod r#impl;

use crate::*;

use std::cmp::{max, min};

type Res<T> = Result<T, Ordering>;

#[derive(Debug, Copy, Clone)]
pub struct NodeInner {
    w: usize,
    l: usize,
    n: usize,
    moves: BoardMove,
    okays: u64,
    proof: Option<Ordering>,
}

#[derive(Debug, Copy, Clone)]
pub struct NodeInfo {
    board: Board,
    parent_side: Side,
    next_move: BoardMove,
    inner: Res<NodeInner>
}

pub struct Node {
    nodes: Vec<Node>,
    info: NodeInfo
}

pub struct Zipper<'a> {
    node: &'a mut Node,
    path: Vec<usize>
}

pub struct Summary {
    pub count: usize,
    pub score: f32,
    pub board: Board,
    pub next_move: BoardMove
}
