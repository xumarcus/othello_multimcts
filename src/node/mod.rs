mod r#impl;

use crate::*;

use std::mem;

use noisy_float::types::{r32, R32};

#[derive(Clone, Debug, Getters)]
pub struct Node {
    nodes: Vec<Node>,
    parent_side: Side,
    info: Result<r#impl::NodeInfo, Winner>,

    #[getset(get = "pub")]
    n: usize,

    #[getset(get = "pub")]
    board: Board,
    
    #[getset(get = "pub")]
    next_move: Moves,
}
