use crate::*;
use super::*;

use std::convert::identity;

impl MCTSInfo {
    fn new(cnt: usize, infos: Vec<NodeInfo>) -> Self {
        MCTSInfo { cnt, infos }
    }

    pub fn best(&self, side: bool) -> Option<NonZeroU64> {
        let info = self.infos.iter().max_by(|a, b| {
            a.avg(side).partial_cmp(&b.avg(side)).unwrap()
        });
        info.and_then(|info| info.next_move)
    }

    pub fn cnt(&self) -> usize {
        self.cnt
    }
}

pub fn seq(board: Board, ms: u64) -> Option<MCTSInfo> {
    let timeup = Arc::new(AtomicBool::new(false));
    /*  simulation is executed sequentially
        no need to lock root with mutex */
    let handle = {
        let timeup = Arc::clone(&timeup);
        thread::spawn(move || {
            let mut root = Node::root(board);
            let mut cnt = 0;
            let mut rng = SmallRng::from_entropy();
            while !timeup.load(Ordering::Acquire) {
                for _ in 0..BLOCK_SIZE {
                    cnt += 1;
                    let (rboard, path) = root
                        .select().expand(&mut rng).consume();
                    let loser = rboard.map_or_else(identity,
                        |board| simulate(board, &mut rng));
                    root.update(loser, path);
                }
            }
            MCTSInfo::new(cnt, root.consume())
        })
    };
    thread::sleep(Duration::from_millis(ms));
    timeup.store(true, Ordering::Release);
    handle.join().ok()
}

pub fn par(board: Board, ms: u64, pthread: usize) -> Option<MCTSInfo> {
    let timeup = Arc::new(AtomicBool::new(false));
    let root = Arc::new(Mutex::new(Node::root(board)));
    let mut handles = Vec::with_capacity(pthread);
    for _ in 0..pthread {
        let timeup = Arc::clone(&timeup);
        let root = Arc::clone(&root);
        handles.push(thread::spawn(move || {
            let mut cnt = 0;
            let mut rng = SmallRng::from_entropy();
            while !timeup.load(Ordering::Acquire) {
                for _ in 0..BLOCK_SIZE {
                    cnt += 1;
                    let (rboard, path) = root.lock().ok()?
                        .select().expand(&mut rng).consume();
                    let loser = rboard.map_or_else(identity,
                        |board| simulate(board, &mut rng));
                    root.lock().ok()?.update(loser, path);
                } 
            }
            Some(cnt)
        }));
    }
    thread::sleep(Duration::from_millis(ms));
    timeup.store(true, Ordering::Release);
    let cnt = handles.into_iter()
        .map(|handle| handle.join().ok().flatten())
        .collect::<Option<Vec<usize>>>()?
        .into_iter()
        .sum();
    Arc::try_unwrap(root).ok()
        .and_then(|mutex| mutex.into_inner().ok())
        .map(|root| MCTSInfo::new(cnt, root.consume()))
}
