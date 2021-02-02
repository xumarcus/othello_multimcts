use super::*;

use std::convert::identity;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering as AtomicOrdering;
use std::time::Duration;
use std::thread;
use rand::SeedableRng;
use rand::rngs::SmallRng;

impl MCTS {
    const BLOCK_SIZE: usize = 100;

    pub fn new(timeout: u64, threads: usize
        , epsilon: f32, algo_type: AlgoType) -> Self
    {
        Self { timeout, threads, epsilon, algo_type }
    }

    pub fn run(&self, board: Board) -> Option<Node> {
        if self.threads != 0 {
            self.par(board)
        } else {
            self.seq(board)
        }
    }

    fn make_algo(&self) -> Algo<SmallRng> {
        Algo::new(self.algo_type, self.epsilon, SmallRng::from_entropy())
    }

    fn seq(&self, board: Board) -> Option<Node> {
        let timeup = Arc::new(AtomicBool::new(false));
        /*  simulation is executed sequentially
            no need to lock root with mutex */
        let handle = {
            let timeup = Arc::clone(&timeup);
            let mut algo = self.make_algo();
            thread::spawn(move || {
                let mut root = Node::root(board);
                while !timeup.load(AtomicOrdering::Acquire) {
                    for _ in 0..MCTS::BLOCK_SIZE {
                        let mut path = Vec::new();
                        let rboard = root
                            .select(&mut path)
                            .expand(&mut path)
                            .get();
                        let ordering = rboard.map_or_else(identity,
                            |board| algo.simulate(board));
                        root.update(ordering, &path);
                    }
                }
                root.best()
            })
        };
        thread::sleep(Duration::from_millis(self.timeout));
        timeup.store(true, AtomicOrdering::Release);
        handle.join().ok()
    }

    fn par(&self, board: Board) -> Option<Node> {
        let timeup = Arc::new(AtomicBool::new(false));
        let root = Arc::new(Mutex::new(Node::root(board)));
        let mut handles = Vec::with_capacity(self.threads);
        for _ in 0..self.threads {
            let timeup = Arc::clone(&timeup);
            let root = Arc::clone(&root);
            let mut algo = self.make_algo();
            handles.push(thread::spawn(move || {
                while !timeup.load(AtomicOrdering::Acquire) {
                    for _ in 0..MCTS::BLOCK_SIZE {
                        let mut path = Vec::new();
                        let rboard = root.lock().ok()?
                            .select(&mut path)
                            .expand(&mut path)
                            .get();
                        let ordering = rboard.map_or_else(identity,
                            |board| algo.simulate(board));
                        root.lock().ok()?.update(ordering, &path);
                    } 
                }
                Some(())
            }));
        }
        thread::sleep(Duration::from_millis(self.timeout));
        timeup.store(true, AtomicOrdering::Release);
        for handle in handles {
            handle.join().ok()??;
        }
        Arc::try_unwrap(root).ok()
            .and_then(|mutex| mutex.into_inner().ok())
            .map(Node::best)
    }
}
