use crate::*;

impl NodeInfo {
    pub fn new(board: Board, m: Option<NonZeroU64>) -> Self {
        NodeInfo { board, m, data: Err((0, 0)) }
    }

    pub fn from_bm((board, m): BM) -> Self {
        NodeInfo::new(board, Some(m))
    }

    pub fn score(&self, side: bool) -> f64 {
        match self.data {
            Err((w, n)) => {
                let avg = (w as f64) / (n as f64);
                if side == self.board.side() { avg } else { 1.0 - avg }
            },
            Ok(loser) => if side == loser { 0.0 } else { 1.0 }
        }
    }

    pub fn score_by_uct(&self, log_n: f64) -> f64 {
        match self.data {
            Err((_, n)) => (2.0 * log_n / (n as f64)).sqrt(),
            _ => 0.0
        }
    }

    pub fn update(&mut self, loser: bool) {
        if let Err((&mut w, &mut n)) = self.info.data.as_mut() {
            n += 1;
            if self.board.side() != loser {
                w += 1;
            }
        }
    }
}
