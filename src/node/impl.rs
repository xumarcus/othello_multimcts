use crate::*;
use rand::Rng;

struct Zipper<'a> {
    node: &'a mut Node,
    path: Vec<&'a mut NodeInfo>,
}

impl Node {
    pub fn new(board: Board) -> Self {
        Node::from_info(NodeInfo::new(board, None))
    }

    fn from_info(info: NodeInfo) -> Self {
        Node {
            nodes: Vec::new(),
            moves: info.board.moves(),
            info
        }
    }

    fn expandable(&self) -> bool {
        self.moves.is_none() && self.info.data.is_err()
    }

    fn side(&self) -> bool {
        self.info.board.side()
    }

    pub fn select<'a>(&'a mut self) -> Zipper<'a> {
        let mut node = self;
        let mut path = vec![&mut self.info];
        while node.moves.is_none() && {
            if let Some(x) = node.by_uct() {
                node = x;
                path.push(&mut x.info);
            } else {
                break;
            }
        }
        Zipper { node, path }
    }

    pub fn expand<'a>(&'a mut self) -> Result<bool, &'a mut Node> {
        
    }

    pub fn propagate(&mut self, loser: bool, path: Vec<&mut NodeInfo>, bm: Option<StateAction>) {
        for info in path {
            info.update(loser);
        }
        if let Some(bm) = bm {
            node.nodes.push(Box::new(Node::new()))
        }
        if let Some((board, m)) = bm {
            let info = NodeInfo::new(board, m);
            node.nodes.push(Box::new(Node::new(m, board)));
        }
        if m.is_some() {
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

    fn by_uct<'a>(&'a mut self) -> Option<&'a mut Node> {
        let (_, n) = self.info.data.err()?;
        let log_n = (n as f64).ln();
        let side = self.side();
        let key = |x: &&'a mut Node| -> f64 {
            x.info.score(side) + x.info.score_by_uct(log_n)
        };
        self.nodes
        .iter_mut()
        .map(Box::as_mut)
        .max_by(|a, b| key(a).partial_cmp(&key(b)).unwrap())
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

fn simulate(board: Board, rng: &mut impl Rng) -> bool {
    let mut tmp = board.clone();
    loop {
        match extract(tmp.moves(), rng) {
            Some(m) => unsafe { tmp = tmp.place_unchecked(m) },
            _ => break tmp.loser()
        };
    }
}
