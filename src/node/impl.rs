use crate::*;
use super::*;
use rand::Rng;

// TODO
pub fn simulate(board: Board, rng: &mut impl Rng) -> bool {
    let mut t = board.clone();
    while let Some(m) = extract(t.moves(), rng) {
        t = t.place(m).unwrap();
    }
    t.loser()
}

impl NodeInnerInfo {
    fn new(moves: NonZeroU64) -> Self {
        NodeInnerInfo { w: 0, n: 0, unused_moves: Some(moves) }
    }

    fn _avg(&self) -> f64 {
        (self.w as f64) / (self.n as f64)
    }

    fn _uct(&self, logn: f64) -> f64 {
        (2.0 * logn / (self.n as f64)).sqrt()
    }
}

impl NodeInfo {
    fn new(board: Board, extracted: NonZeroU64) -> Self {
        NodeInfo::from_opt(board, Some(extracted))
    }

    fn from_opt(board: Board, next_move: Option<NonZeroU64>) -> Self {
        NodeInfo {
            board,
            next_move,
            inner_info: board.moves()
                .ok_or_else(|| board.loser())
                .map(NodeInnerInfo::new)
        }
    }

    pub fn avg(&self, side: bool) -> f64 {
        match self.inner_info.as_ref() {
            Ok(info) => if side == self.board.side()
                { info._avg() } else { 1.0 - info._avg() },
            Err(&loser) => if side == loser
                { 0.0 } else { 1.0 }
        }
    }

    fn uct(&self, logn: f64) -> f64 {
        self.inner_info.as_ref().map_or(0.0, |info| info._uct(logn))
    }
}

impl Node {
    pub fn root(board: Board) -> Self {
        Node::new(NodeInfo::from_opt(board, None))
    }
    
    pub fn new(info: NodeInfo) -> Self {
        Node { nodes: Vec::new(), info }
    }

    pub fn select<'a>(&'a mut self) -> Zipper<'a> {
        let mut node = self;
        let mut path = Vec::new();
        while let Ok(info) = node.info.inner_info.as_ref() {
            if let Some(_) = info.unused_moves {
                break;
            }
            if let Some(i) = node.best_index() {
                path.push(i);
                node = &mut node.nodes[i]; // NLL
            } else {
                break;
            }
        }
        Zipper { node, path }
    }

    fn side(&self) -> bool {
        self.info.board.side()
    }

    fn best_index(&mut self) -> Option<usize> {
        let logn = (self.info.inner_info.as_ref().ok()?.n as f64).ln();
        let side = self.side();
        let keyf = |x: &Box<Node>| x.info.avg(side) + x.info.uct(logn);
        self.nodes.iter().enumerate().max_by(|(_, a), (_, b)| {
            keyf(a).partial_cmp(&keyf(b)).unwrap()
        }).map(|p| p.0)
    }

    pub fn consume(self) -> Vec<NodeInfo> {
        self.nodes
        .into_iter()
        .map(|node| node.info)
        .collect()
    }

    pub fn update(&mut self, loser: bool, path: Vec<usize>) {
        let mut node = &mut *self;
        let mut path_it = path.iter();
        let provable_loser = loop {
            let side = node.side();
            match node.info.inner_info.as_mut() {
                Err(&mut pl) => break pl,
                Ok(info) => {
                    info.n += 1;
                    if side != loser {
                        info.w += 1;
                    }
                }
            }
            if let Some(&index) = path_it.next() {
                node = &mut node.nodes[index];
            } else {
                return;
            }
        };
        node = &mut *self;
        let mut ils = Vec::new();
        for index in path {
            if node.info.inner_info.is_err() {
                break;
            }
            let side = node.side();
            let can_lose = node.nodes.iter()
                .enumerate()
                .all(|(i, x)| {
                    if i == index {
                        true
                    } else if let Err(l) = x.info.inner_info {
                        l == side
                    } else {
                        false
                    }
                });
            ils.push((&mut node.info, can_lose));
            node = node.nodes[index].as_mut();
        }
        for (info, can_lose) in ils.into_iter().rev() {
            if provable_loser == info.board.side() && !can_lose {
                break;
            }
            info.inner_info = Err(loser);
        }
    }
}

impl<'a> Zipper<'a> {
    // Lifetime conflict if nodes altered between expansion and consumption
    pub fn expand(self, rng: &mut impl Rng) -> Zipper<'a> {
        let board = &self.node.info.board;
        if let Ok(inner_info) = &mut self.node.info.inner_info {
            let unused = &mut inner_info.unused_moves;
            if let Some(extracted) = extract(*unused, rng) {
                let rem = unused.unwrap().get() - extracted.get();
                *unused = NonZeroU64::new(rem);
                let new_b = board.place(extracted).unwrap();
                let new_i = NodeInfo::new(new_b, extracted);
                let nodes = &mut self.node.nodes;
                let mut path = self.path.to_owned();
                path.push(nodes.len());
                nodes.push(Box::new(Node::new(new_i)));
                let node = nodes.last_mut().unwrap();
                return Zipper { node, path };
            }
        }
        self
    }

    pub fn consume(&'a mut self) -> (Result<Board, bool>, Vec<usize>) {
        let info = &self.node.info;
        (info.inner_info.as_ref()
         .map(|_| info.board).map_err(|&l| l), self.path.to_owned())
    }

}

fn extract(moves: Option<NonZeroU64>, rng: &mut impl Rng) -> Option<NonZeroU64> {
    moves.map(NonZeroU64::get)
    .and_then(|moves| {
        let n = moves.count_ones() as usize;
        (0..64)
        .map(|i| 1u64 << i)
        .filter(|m| m & moves != 0)
        .nth(rng.gen_range(0..n))
        .and_then(NonZeroU64::new)
    })
}
