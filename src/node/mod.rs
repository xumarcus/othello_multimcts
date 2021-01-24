mod r#impl;

use crate::*;

pub struct Node {
    nodes: Vec<Box<Node>>,
    moves: Option<NonZeroU64>,
    info: NodeInfo
}
