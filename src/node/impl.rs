use crate::Board;
use crate::mcts::node::Node;
use crate::mcts::node::{SR0, SR1, SR2};

use rand::seq::IteratorRandom;

use std::convert::identity;

impl SR0 {
    pub fn run(self) -> SR1 {
        let loser = self.res.map_or_else(identity, |board| {
            let mut tmp = board.clone();
            loop {
                let m = extract(tmp.moves());
                assert_ne!(m, 0);
                match tmp.place(m) {
                    Ok(nboard) => { tmp = nboard; },
                    Err(loser) => { break loser }
                }
            }
        });
        SR1 {
            path: self.path,
            mmask: self.mmask,
            loser,
            optbd: self.res.ok(),
        }
    }
}

impl Node {
    pub fn new(board: Board) -> Node {
        Node {
            board,
            mmask: 0,
            moves: board.moves(),
            nwins: 0,
            visit: 0,
            nodes: Vec::new(),
        }
    }

    pub fn select(&mut self) -> SR0 {
        let mut node = self;
        let mut path = Vec::new();
        loop {
            match extract(node.moves) {
                0 => match node.uct_select() {
                    // TODO
                    Ok((i, x)) => {
                        node = x;
                        path.push(i);
                    },
                    Err(loser) => break SR0 {
                        path,
                        mmask: 0,
                        res: Err(loser)
                    }
                }
                m => {
                    node.moves -= m;
                    break SR0 {
                        path,
                        mmask: m,
                        res: node.board.place(m)
                    };
                }
            }
        }
    }

    pub fn propagate(&mut self, SR1 { path, mmask, loser, optbd }: SR1) {
        self.update(loser);
        let mut node = self;
        for idx in path {
            node = node.nodes[idx].as_mut();
            node.update(loser);
        }
        if let Some(board) = optbd {
            let new_node = Node {
                mmask,
                nwins: (board.side() != loser).into(),
                visit: 1,
                ..Node::new(board)
            };
            node.nodes.push(Box::new(new_node));
        }
    }

    pub fn run(&self) -> Vec<SR2> {
        self.nodes.iter().map(|node| SR2 {
            board: node.board,
            mmask: node.mmask,
            score: node.score(self.board.side()),
        }).collect()
    }

    fn update(&mut self, loser: bool) {
        self.visit += 1;
        if self.board.side() != loser {
            self.nwins += 1;
        }
    }

    fn uct_select<'a>(&'a mut self) -> Result<(usize, &'a mut Node), bool> {
        let side = self.board.side();
        let log_n = (self.visit as f64).ln();
        self.nodes
            .iter_mut()
            .map(Box::as_mut)
            .enumerate()
            .max_by(|(_, a), (_, b)| {
            let sa = a.score(side) + a.uctpv(log_n);
            let sb = b.score(side) + b.uctpv(log_n);
            sa.partial_cmp(&sb).unwrap()
        }).ok_or(self.board.current_loser())
    }

    fn score(&self, side: bool) -> f64 {
        let w = self.nwins as f64;
        let n = self.visit as f64;
        if self.board.side() == side { w / n } else { 1.0 - w / n }
    }

    fn uctpv(&self, log_n: f64) -> f64 {
        let n = self.visit as f64;
        (2.0 * log_n / n).sqrt()
    }
}

fn extract(moves: u64) -> u64 {
    let mut rng = rand::thread_rng();
    (0..64)
        .map(|i| 1 << i)
        .filter(|m| m & moves != 0)
        .choose(&mut rng)
        .unwrap_or(0)
}
