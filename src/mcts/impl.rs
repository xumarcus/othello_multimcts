use super::*;

pub fn seq(board: Board, ms: u64) -> Option<(usize, Vec<NextMove>)> {
    let timeup = Arc::new(AtomicBool::new(false));
    /*  simulation is executed sequentially
        no need to lock root with mutex */
    let handle = {
        let timeup = Arc::clone(&timeup);
        thread::spawn(move || {
            let mut root = Node::new(board);
            let mut cnt = 0;
            let mut rng = SmallRng::from_entropy();
            while !timeup.load(Ordering::Acquire) {
                for _ in 0..100 {
                    let (res, path) = root.select(&mut rng);
                    let rr = run(res, &mut rng);
                    root.propagate(rr, path);
                    cnt += 1;
                }
            }
            (cnt, root.run())
        })
    };
    thread::sleep(Duration::from_millis(ms));
    timeup.store(true, Ordering::Release);
    handle.join().ok()
}

pub fn par(board: Board, ms: u64, pthread: usize) -> Option<(usize, Vec<NextMove>)> {
    let timeup = Arc::new(AtomicBool::new(false));
    let root = Arc::new(Mutex::new(Node::new(board)));
    let mut handles = Vec::with_capacity(pthread);
    for _ in 0..pthread {
        let timeup = Arc::clone(&timeup);
        let root = Arc::clone(&root);
        handles.push(thread::spawn(move || {
            let mut cnt = 0;
            let mut rng = SmallRng::from_entropy();
            while !timeup.load(Ordering::Acquire) {
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
