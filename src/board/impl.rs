use crate::board::Board;

impl Board {
    const DEFAULT_SIDE: bool = false;

    pub fn new(me: u64, op: u64) -> Option<Board> {
        match me & op {
            0 => Some(Board {
                me,
                op,
                side: Board::DEFAULT_SIDE,
            }),
            _ => None,
        }
    }

    pub fn side(&self) -> bool {
        self.side
    }

    pub fn current_loser(&self) -> bool {
        self.side ^ (self.me.count_ones() > self.op.count_ones())
    }

    pub fn moves(&self) -> u64 {
        Board::DIRS
            .iter()
            .map(|f| f(self.me, self.op))
            .fold(0, |a, x| a | x)
            & !(self.me | self.op)
    }

    // undefined behavior if m is illegal
    // losing side if no more moves

    pub fn place(&self, m: u64) -> Result<Board, bool> {
        let cap = Board::DIRS
            .iter()
            .map(|f| f(m, self.op))
            .filter(|g| g & self.me != 0)
            .fold(0, |a, x| a | x);
        let new_me = self.me | m | cap;
        let new_op = self.op & !cap;
        let diff_side = Board {
            me: new_op,
            op: new_me,
            side: !self.side,
        };
        let same_side = Board {
            me: new_me,
            op: new_op,
            side: self.side,
        };
        if diff_side.moves() != 0 {
            Ok(diff_side)
        } else if same_side.moves() != 0 {
            Ok(same_side)
        } else {
            Err(same_side.current_loser())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easy() {
        assert_eq!(
            (Board {
                me: 0x0000000020040000,
                op: 0x00007f1c1c100000,
            })
            .moves(),
            /**/ 0x0044000002000800
        );
    }

    #[test]
    fn test_hard() {
        assert_eq!(
            (Board {
                me: 0x0000f80006140800,
                op: 0x8844073c38202440,
            })
            .moves(),
            /**/ 0x6601004040480224
        );
    }
}
