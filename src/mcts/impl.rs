use crate::*;
use super::*;

impl MCTS {
    const BLOCK_SIZE: usize = 100;

    pub fn run(&self, board: Board) -> Option<Summary> {
        if self.threads != 0 {
            self.par(board)
        } else {
            self.seq(board)
        }
    }

    fn make_algo(&self) -> Algo<SmallRng> {
        Algo::new(self.algo_type, self.epsilon, SmallRng::from_entropy())
    }

    fn seq(&self, board: Board) -> Option<Summary> {
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
                        let (rboard, path) = root
                            .select()
                            .expand()
                            .consume();
                        let ordering = rboard.map_or_else(identity,
                            |board| algo.simulate(board));
                        root.update(ordering, path);
                    }
                }
                root.summarize()
            })
        };
        thread::sleep(Duration::from_millis(self.timeout));
        timeup.store(true, AtomicOrdering::Release);
        handle.join().ok().flatten()
    }

    fn par(&self, board: Board) -> Option<Summary> {
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
                        let (rboard, path) = root.lock().ok()?
                            .select()
                            .expand()
                            .consume();
                        let ordering = rboard.map_or_else(identity,
                            |board| algo.simulate(board));
                        root.lock().ok()?.update(ordering, path);
                    } 
                }
                Some(())
            }));
        }
        thread::sleep(Duration::from_millis(self.timeout));
        timeup.store(true, AtomicOrdering::Release);
        for handle in handles {
            handle.join().ok().flatten()?;
        }
        Arc::try_unwrap(root).ok()
            .and_then(|mutex| mutex.into_inner().ok())
            .and_then(Node::summarize)
    }
}