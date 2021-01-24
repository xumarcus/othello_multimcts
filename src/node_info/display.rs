use crate::*;

use std::fmt;

impl fmt::Display for NodeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Board:\n{}", self.board);
        writeln!(f, "Score: {:.1}%", self.score() * 100.0);
        writeln!(f, "N: {}", self.n);
        if let Some(m) = self.m {
            writeln!(f, "Move: [{}]", Move(m));
        }
    }
}
