mod r#impl;

use crate::*;

#[derive(Debug)]
pub struct NodeInnerInfo {
    w: usize,
    n: usize,
    unused_moves: Option<NonZeroU64>
}

#[derive(Debug)]
pub struct NodeInfo {
    pub board: Board,
    pub next_move: Option<NonZeroU64>,
    inner_info: Result<NodeInnerInfo, bool>
}

pub struct Node {
    nodes: Vec<Box<Node>>,
    info: NodeInfo
}

pub struct Zipper<'a> {
    node: &'a mut Node,
    path: Vec<usize>
}

pub use r#impl::simulate;
