use super::*;

impl AI for MCTS {
    fn run(&mut self) -> Failable<Moves> {
        let count = match self.config.threads {
            0 => self.seq()?,
            _ => self.par()?
        };
        debug!("Count: {}", count);

        // self.root is replaced with best node
        // where parent_side is player's side
        // since ai is not run when there are no moves 
        debug!("Prob: {:.1}%", self.root.avg() * 100.0);
        Ok(*self.root.next_move())
    }

    // Regenerate board for white AI
    // Since Node::new is cheap
    fn update(&mut self, next_move: Moves) {
        if !next_move.is_nonzero() {
            return;
        }
        if self.root.next_move().is_nonzero() {
            self.root.place(next_move);
        } else {
            let board = self.root.board().place(next_move);
            self.root = Node::new(board);
        }
    }
}

impl MCTS {
    const BLOCK_SIZE: usize = 100;

    pub fn new(config: Config) -> Self {
        Self { root: Node::new(config.board), config }
    }

    fn algo(&self) -> Algo<SmallRng> {
        Algo::new(
            self.config.algo_type,
            self.config.epsilon,
            SmallRng::from_entropy()
        )
    }

    fn seq(&mut self) -> Failable<usize> {
        let timeup = Arc::new(AtomicBool::new(false));
        /*  simulation is executed sequentially
            no need to lock root with mutex */
        let handle = {
            // move root from self into thread
            // nodes are allocated on heap
            let mut root = mem::take(&mut self.root);
            let mut algo = self.algo();
            let timeup = Arc::clone(&timeup);

            thread::spawn(move || {
                while !timeup.load(AtomicOrdering::Acquire) {
                    for _ in 0..MCTS::BLOCK_SIZE {
                        MCTSRunner::new(&mut root)
                            .run_sim(&mut algo)
                            .run_update(&mut root);
                    }
                }
                root
            })
        };
        thread::sleep(Duration::from_millis(self.config.timeout));
        timeup.store(true, AtomicOrdering::Release);
        self.root = handle.join().expect("Expect no panic");
        Ok(self.root.place_best())
    }

    // TODO handle thread kill
    fn par(&mut self) -> Failable<usize> {
        let timeup = Arc::new(AtomicBool::new(false));
        let root = Arc::new(Mutex::new(mem::take(&mut self.root)));
        let mut handles = Vec::with_capacity(self.config.threads);
        for _ in 0..self.config.threads {
            let timeup = Arc::clone(&timeup);
            let root = Arc::clone(&root);
            let mut algo = self.algo();
            handles.push(thread::spawn(move || {
                while !timeup.load(AtomicOrdering::Acquire) {
                    for _ in 0..MCTS::BLOCK_SIZE {
                        let runner = MCTSRunner::new(&mut root.lock().unwrap());
                        runner.run_sim(&mut algo)
                            .run_update(&mut root.lock().unwrap());
                    } 
                }
            }));
        }
        thread::sleep(Duration::from_millis(self.config.timeout));
        timeup.store(true, AtomicOrdering::Release);
        for handle in handles {
            handle.join().expect("Expect no panic");
        }
        let mutex = Arc::try_unwrap(root)
            .expect("Children should not have strong ref");
        self.root = mutex.into_inner()
            .expect("Expect no panic");
        Ok(self.root.place_best())
    }

    // TODO par_unseq
}
