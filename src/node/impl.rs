use crate::*;
use super::*;

impl NodeInner {
    fn new(moves: BoardMove) -> Self {
        NodeInner {
            w: 0,
            l: 0,
            n: 0,
            moves,
            okays: !(u64::MAX.wrapping_shl(moves.0.count_ones())),
            proof: None
        }
    }
}

impl NodeInfo {
    fn new(board: Board, parent_side: Side, next_move: BoardMove) -> Self {
        let inner = match board.moves() {
            BoardMove(0) => Err(board.ordering()),
            moves        => Ok(NodeInner::new(moves))
        };
        NodeInfo { board, parent_side, next_move, inner }
    }

    fn avg(&self) -> f32 {
        match self.inner.as_ref() {
            Ok(inner) => match inner.w + inner.l {
                0 => 0.5,
                t => (inner.w as f32) / (t as f32)
            },
            Err(ordering) => match self.parent_side {
                Side::Black => match ordering {
                    Ordering::Less => 0.0,
                    Ordering::Equal => 0.5,
                    Ordering::Greater => 1.0
                },
                Side::White => match ordering {
                    Ordering::Less => 1.0,
                    Ordering::Equal => 0.5,
                    Ordering::Greater => 0.0
                }
            }
        }
    }

    fn uct(&self, lognum: f32) -> f32 {
        self.inner.as_ref().map_or(0.0, |inner| {
            f32::sqrt(2.0 * lognum / (inner.n as f32))
        })
    }

    fn update(&mut self, ordering: Ordering) {
        let inner = self.inner.as_mut().unwrap();
        match self.parent_side {
            Side::Black => match ordering {
                Ordering::Less => inner.l += 1,
                Ordering::Greater => inner.w += 1,
                _ => ()
            },
            Side::White => match ordering {
                Ordering::Less => inner.w += 1,
                Ordering::Greater => inner.l += 1,
                _ => ()
            }
        }
    }
}

impl Node {
    pub fn root(board: Board) -> Self {
        Node::new(NodeInfo::new(board, board.side(), BoardMove(0)))
    }
    
    pub fn select(&mut self) -> Zipper {
        let mut node = self;
        let mut path = Vec::new();
        while let Ok(inner) = node.info.inner.as_mut() {
            inner.n += 1;
            if inner.moves.0 != 0 {
                break;
            }
            let lognum = f32::ln(inner.n as f32);
            let idx = node.nodes.iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| {
                    a.val(lognum).partial_cmp(&b.val(lognum))
                    .expect("Is not NaN")
                })
                .map(|p| p.0)
                .expect("At least one node in Vec exists");
            path.push(idx);
            node = &mut node.nodes[idx];
        }
        Zipper { node, path }
    }

    pub fn summarize(self) -> Option<Summary> {
        self.nodes.iter()
        .map(|node| node.info)
        .max_by(|a, b| {
            a.avg().partial_cmp(&b.avg()).expect("Is not NaN")
        })
        .map(|info| Summary {
            count: self.info.inner.map_or(0, |inner| inner.n),
            score: info.avg(),
            board: info.board,
            next_move: info.next_move
        })
    }

    pub fn update(&mut self, ordering: Ordering, path: Vec<usize>) {
        if self.info.inner.is_ok() {
            self.info.update(ordering);
        } else {
            return; // Nothing to do
        }
        let mut v = Vec::new();
        let mut node = &mut *self;
        for index in path {
            v.push((index, &mut node.info));
            node = &mut node.nodes[index];
            if let Err(ord) = node.info.inner {
                for (index, info) in v.iter_mut().rev() {
                    let mask = 1u64 << *index;
                    let inner = info.inner.as_mut()
                        .expect("Loop return on Err");
                    if inner.okays & mask != 0 {
                        inner.okays -= mask;
                        let was = inner.proof.get_or_insert(ord);
                        match info.parent_side {
                            Side::Black => *was = max(*was, ord),
                            Side::White => *was = min(*was, ord)
                        }
                    }
                    if inner.okays != 0 {
                        break;
                    } else {
                        info.inner = Err(ord);
                    }
                }
                break;
            } else {
                node.info.update(ordering);
            }
        }
    }

    fn new(info: NodeInfo) -> Self {
        Node { nodes: Vec::new(), info }
    }

    fn val(&self, lognum: f32) -> f32 {
        self.info.avg() + self.info.uct(lognum)
    }
}

impl<'a> Zipper<'a> {
    // Lifetime conflict if nodes altered between expansion and consumption
    // Therefore return new Zipper
    pub fn expand(self) -> Zipper<'a> {
        let board = &self.node.info.board;
        if let Ok(inner) = &mut self.node.info.inner {
            let m = &mut inner.moves.0;
            if *m != 0 {
                let x = *m & (!(*m) + 1);
                *m -= x;
                let next_move = BoardMove(x);
                let new_b = board.place(next_move).unwrap();
                let new_i = NodeInfo::new(new_b, board.side(), next_move);
                let nodes = &mut self.node.nodes;
                let mut path = self.path.to_owned();
                path.push(nodes.len());
                nodes.push(Node::new(new_i));
                let node = nodes.last_mut().unwrap();
                return Zipper { node, path };
            }
        }
        self
    }

    // Rust has no BiMap :(
    pub fn consume(self) -> (Res<Board>, Vec<usize>) {
        let info = &self.node.info;
        let res = match info.inner.as_ref() {
            Err(&ordering) => Err(ordering),
            Ok(_) => Ok(info.board)
        };
        (res, self.path.to_owned())
    }
}

/*
// lsb is faster
fn msb(mut x: u64) -> u64 {
    x |= x >> 1;
    x |= x >> 2;
    x |= x >> 4;
    x |= x >> 8;
    x |= x >> 16;
    x |= x >> 32;
    x += 1;
    if x != 0 {
        x >> 1
    } else {
        1u64 << 63
    }
}
*/