use crate::mcts::NextMove;

use std::cmp::{Eq, PartialOrd, Ord, Ordering};

impl Eq for NextMove { /* nothing */ }

impl PartialOrd for NextMove {
    fn partial_cmp(&self, other:& Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for NextMove {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
