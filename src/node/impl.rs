use super::*;

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct NodeInfo {
    parent_side: Side,
    moves: Moves,
    okays: u64,
    w: usize,
    l: usize,
    proof: Option<Winner>,
}

impl NodeInfo {
    fn new(parent_side: Side, moves: Moves) -> Self {
        let okays = !(u64::MAX.wrapping_shl(moves.0.count_ones()));
        Self { parent_side, moves, okays, ..Self::default() }
    }

    #[inline]
    fn update(&mut self, winner: Winner) {
        if let Some(winner) = winner {
            if self.parent_side == winner {
                self.w += 1;
            } else {
                self.l += 1;
            }
        }
    }

    fn update_proof(&mut self, index: usize, proof: Winner) {
        let mask = 1u64 << index;
        if self.okays & mask != 0 {
            self.okays -= mask;
            if let Some(pf) = self.proof {
                let white_should_update = match (pf, proof) {
                    (Some(Side::Black), _) => true,
                    (_, Some(Side::Black)) => false,
                    (Some(Side::White), _) => false,
                    (_, Some(Side::White)) => true,
                    _ => false
                };
                if white_should_update == (self.parent_side == Side::Black) {
                    return;
                }
            }
            self.proof = Some(proof);
        }
    }
}

impl Node {
    pub fn root(board: Board) -> Self {
        Node::new(*board.side(), board, Moves(0))
    }

    pub fn place(&mut self, next_move: Moves) {
        if let Some(node) = mem::take(&mut self.nodes)
            .into_iter()
            .find(|node| node.next_move == next_move)
        {
            *self = node;
        }
    }

    pub fn place_best(&mut self) -> usize {
        mem::take(&mut self.nodes)
        .into_iter()
        .max_by_key(Node::avg)
        .map(|best| {
            let n = self.n;
            *self = best;
            n
        })
        .unwrap_or_default()
    }

    pub fn avg(&self) -> R32 {
        let avg = match self.info.as_ref() {
            Ok(info) => match info.w + info.l {
                0 => 0.5,
                t => (info.w as f32) / (t as f32)
            },
            Err(winner) => winner.map_or(0.5, |side| {
                match side == self.parent_side {
                    true => 1.0,
                    _ => 0.0
                }
            })
        };
        r32(avg)
    }

    fn select(&mut self, path: &mut Vec<usize>) -> &mut Node {
        self.n += 1;
        if let Ok(info) = self.info.as_mut() {
            if !info.moves.is_nonzero() {
                let lognum = f32::ln(self.n as f32);
                debug_assert!(!lognum.is_nan());
                let index = self.nodes.iter()
                    .enumerate()
                    .max_by_key(|(_, node)| node.val(lognum))
                    .map(|(i, _)| i)
                    .expect("At least one node"); // TODO
                path.push(index);
                return self.nodes[index].select(path);
            }
        }
        self
    }

    fn expand(&mut self, path: &mut Vec<usize>) -> &mut Node {
        if let Ok(info) = self.info.as_mut() {
            // next(&mut self) modifies info.moves
            if let Some(next_move) = info.moves.next() {
                let index = self.add_child(next_move);
                path.push(index);
                return &mut self.nodes[index];
            }
        }
        self
    }

    fn update(&mut self, winner: Winner, path: &[usize]) -> Option<Winner> {
        match self.info.as_mut() {
            Err(proof) => return Some(*proof),
            Ok(info) => {
                info.update(winner);
                let (index, tail) = path.split_first()?;
                let proof = self.nodes[*index].update(winner, tail)?;
                info.update_proof(*index, proof);
            }
        };
        let proof: Winner = self.info.ok().filter(|info| info.okays == 0)?.proof?;
        self.info = Err(proof);
        Some(proof)
    }

    fn add_child(&mut self, next_move: Moves) -> usize {
        let index = self.nodes.len();
        self.nodes.push(Node::new(*self.board.side(), self.board, next_move));
        index
    }

    fn new(parent_side: Side, board: Board, next_move: Moves) -> Self {
        let board = match next_move {
            Moves(0) => board,
            next_move => board.place(next_move)
        };
        let info = match board.moves() {
            Moves(0) => Err(board.winner()),
            moves => Ok(NodeInfo::new(parent_side, *moves))
        };
        Node { nodes: Vec::new(), parent_side, info, n: 0, board, next_move }
    }

    fn val(&self, lognum: f32) -> R32 {
        let uct = f32::sqrt(2.0 * lognum / (self.n as f32));
        self.avg() + r32(uct)
    }
}

impl MCTSRunner {
    pub fn new(root: &mut Node) -> Self {
        let mut path = Vec::new();
        let node = root
            .select(&mut path)
            .expand(&mut path);
        let board = node.info
            .map(|_| node.board);
        Self { board, path }
    }

    pub fn run_sim<T: Rng>(mut self, algo: &mut Algo<T>) -> Self {
        if let Ok(board) = self.board {
            self.board = Err(algo.simulate(board));
        }
        self
    }

    pub fn run_update(&self, root: &mut Node) {
        root.update(self.board.unwrap_err(), self.path.as_slice());
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
