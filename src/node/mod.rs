mod r#impl;

use crate::*;

use std::mem;

use noisy_float::types::{r32, R32};

#[derive(Clone, Debug, Derivative, Getters)]
#[derivative(Default)]
pub struct Node {
    nodes: Vec<Node>,
    parent_side: Side,

    #[derivative(Default(value="Ok(Default::default())"))]
    info: Result<r#impl::NodeInfo, Winner>,

    #[getset(get = "pub")]
    n: usize,

    #[getset(get = "pub")]
    board: Board,
    
    #[getset(get = "pub")]
    next_move: Moves,
}

#[derive(Debug)]
pub struct MCTSRunner {
    board: Result<Board, Winner>,
    path: Vec<usize>
}
