mod r#impl;

use crate::Board;

#[derive(Debug)]
pub struct Node {
    board: Board,
    mmask: u64,
    moves: u64,
    nwins: usize,
    visit: usize,
    nodes: Vec<Box<Node>>,
}

#[derive(Debug)]
pub struct SR0 {
    path: Vec<usize>,
    mmask: u64,
    res: Result<Board, bool>,
}

#[derive(Debug)]
pub struct SR1 {
    path: Vec<usize>,
    mmask: u64,
    loser: bool,
    optbd: Option<Board>,
}

#[derive(Debug)]
pub struct SR2 {
    pub board: Board,
    pub mmask: u64,
    pub score: f64
}

use std::time::Instant;

pub fn mcts(board: Board, ms: u128) -> Vec<SR2> {
    let mut root = Node::new(board);
    let now = Instant::now(); 
    while now.elapsed().as_millis() < ms {
        let sr0 = root.select();
        let sr1 = sr0.run();
        root.propagate(sr1);
    }
    root.run()
}
