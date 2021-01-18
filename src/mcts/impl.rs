use crate::Board;
use crate::mcts::{Node, NextMove};

use std::num::NonZeroU64;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::thread;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

pub enum FromRun {
    NoMove,
    NoBoard(NonZeroU64),
    HasBoth(NonZeroU64, Board)
}

type NextBoard = Result<bool, (NonZeroU64, Board)>;
type RunResult = (bool, FromRun);

#[allow(dead_code)]
pub fn mcts_seq(board: Board, ms: u64) -> Option<(usize, Vec<NextMove>)> {
    let timeup = Arc::new(AtomicBool::new(false));
    let time_t = Arc::clone(&timeup);
    let handle = thread::spawn(move || {
        let mut root = Node::new(board);
        let mut cnt = 0;
        let mut rng = SmallRng::from_entropy();
        while !time_t.load(Ordering::Acquire) {
            for _ in 0..100 {
                let (res, path) = root.select(&mut rng);
                let rr = run(res, &mut rng);
                root.propagate(rr, path);
                cnt += 1;
            }
        }
        (cnt, root.run())
    });
    thread::sleep(Duration::from_millis(ms));
    timeup.store(true, Ordering::Release);
    handle.join().ok()
}

#[allow(dead_code)]
pub fn mcts_par(board: Board, ms: u64, pthread: usize) -> Option<(usize, Vec<NextMove>)> {
    let timeup = Arc::new(AtomicBool::new(false));
    let root = Arc::new(Mutex::new(Node::new(board)));
    let mut handles = Vec::with_capacity(pthread);
    for _ in 0..pthread {
        let time_t = Arc::clone(&timeup);
        let root = Arc::clone(&root);
        handles.push(thread::spawn(move || {
            let mut cnt = 0;
            let mut rng = SmallRng::from_entropy();
            while !time_t.load(Ordering::Acquire) {
                for _ in 0..100 {
                    let (res, path) = root.lock().ok()?.select(&mut rng);
                    let rr = run(res, &mut rng);
                    root.lock().ok()?.propagate(rr, path);
                    cnt += 1;
                } 
            }
            Some(cnt)
        }));
    }
    thread::sleep(Duration::from_millis(ms));
    timeup.store(true, Ordering::Release);
    let mut cnt = 0;
    for handle in handles {
        cnt += handle.join().ok().flatten()?;
    }
    Arc::try_unwrap(root).ok()
        .and_then(|mutex| mutex.into_inner().ok())
        .map(|root| (cnt, root.run()))
}

impl Node {
    pub fn new(board: Board) -> Node {
        Node {
            board,
            moves: Some(board.moves()),
            mmask: None,
            nwins: 0,
            visit: 0,
            nodes: Vec::new(),
        }
    }

    pub fn select(&mut self, rng: &mut impl Rng) -> (NextBoard, Vec<usize>) {
        let mut node = self;
        let mut path = Vec::new();
        loop {
            match node.moves {
                None => match node.uct_select() {
                    // TODO
                    Err((i, x)) => {
                        node = x;
                        path.push(i);
                    },
                    Ok(loser) => break (Ok(loser), path)
                }
                Some(moves) => {
                    let m = extract_unchecked(moves, rng);
                    node.moves = NonZeroU64::new(moves.get() - m.get());
                    break (Err((m, node.board)), path);
                }
            }
        }
    }

    pub fn propagate(&mut self, (loser, frun): RunResult, path: Vec<usize>) {
        self.update(loser);
        let mut node = self;
        for idx in path {
            node = node.nodes[idx].as_mut();
            node.update(loser);
        }
        if let FromRun::HasBoth(mmask, board) = frun {
            let new_node = Node {
                mmask: Some(mmask),
                nwins: (board.side() != loser).into(),
                visit: 1,
                ..Node::new(board)
            };
            node.nodes.push(Box::new(new_node));
        }
    }

    pub fn run(&self) -> Vec<NextMove> {
        self.nodes.iter().map(|node| NextMove {
            board: node.board,
            mmask: node.mmask.expect("Cannot be root"),
            score: node.score(self.board.side()),
        }).collect()
    }

    fn update(&mut self, loser: bool) {
        self.visit += 1;
        if self.board.side() != loser {
            self.nwins += 1;
        }
    }

    fn uct_select<'a>(&'a mut self) -> Result<bool, (usize, &'a mut Node)> {
        let side = self.board.side();
        let log_n = (self.visit as f64).ln();
        let loser = self.board.loser();
        self.nodes
            .iter_mut()
            .map(Box::as_mut)
            .enumerate()
            .max_by(|(_, a), (_, b)| {
            let sa = a.score(side) + a.uctpv(log_n);
            let sb = b.score(side) + b.uctpv(log_n);
            sa.partial_cmp(&sb).unwrap()
        }).map_or(Ok(loser), Err)
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

/*
fn extract(moves: Option<NonZeroU64>, rng: &mut impl Rng) -> Option<NonZeroU64> {
    moves.map(|moves| extract_unchecked(moves, rng))
}
*/

fn extract_unchecked(moves: NonZeroU64, rng: &mut impl Rng) -> NonZeroU64 {
    let moves = moves.get();
    let mut n = rng.gen_range(0..moves.count_ones());
    for i in 0..64 {
        let m = 1u64 << i;
        if m & moves != 0 {
            if n != 0 {
                n -= 1;
            } else {
                unsafe { return NonZeroU64::new_unchecked(m); }
            }
        }
    }
    panic!();
}

fn simulate(board: Board, rng: &mut impl Rng) -> bool {
    let mut tmp = board.clone();
    loop {
        match tmp.place(extract_unchecked(tmp.moves(), rng)) {
            Ok(loser) => { break loser; },
            Err(board) => { tmp = board; },
        }
    }
}

fn run(res: NextBoard, rng: &mut impl Rng) -> RunResult {
    match res {
        Ok(loser) => (loser, FromRun::NoMove),
        Err((m, b)) => match b.place(m) {
            Ok(loser) => (loser, FromRun::NoBoard(m)),
            Err(next) => (simulate(next, rng), FromRun::HasBoth(m, next))
        }
    }
}

