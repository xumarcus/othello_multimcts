use crate::*;

impl Board {
    pub fn new() -> Board {
        let me = 0x0000_0008_1000_0000;
        let op = 0x0000_0010_0800_0000;
        Board::make(me, op, bool::default()).unwrap()
    }

    pub fn make(me: u64, op: u64, side: bool) -> Option<Board> {
        Some(Board {
            me, op, side,
            moves: NonZeroU64::new(moves(me, op))
        }).filter(|x| x.me & x.op == 0)
    }

    pub fn side(&self) -> bool {
        self.side
    }

    pub fn moves(&self) -> Option<NonZeroU64> {
        self.moves
    }

    pub fn loser(&self) -> bool {
        self.side() ^ (self.me.count_ones() > self.op.count_ones())
    }

    // TODO
    pub fn place(&self, m: NonZeroU64) -> Option<Board> {
        let c = Board::DIRS
            .iter()
            .map(|f| f(m.get(), self.op))
            .filter(|g| g & self.me != 0)
            .fold(0, |a, x| a | x);
        let me = self.me | (m.get() | c);
        let op = self.op & !c;
        Board::make(op, me, !self.side)
            .filter(|df| df.moves.is_some())
            .or_else(|| Board::make(me, op, self.side))
    }
}

fn moves(me: u64, op: u64) -> u64 {
    Board::DIRS
        .iter()
        .map(|f| f(me, op))
        .fold(0, |a, x| a | x)
        & !(me | op)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easy() {
        assert_eq!(moves(
            0x0000000020040000,
            0x00007f1c1c100000
        ),
            0x0044000002000800
        );
    }

    #[test]
    fn test_hard() {
        assert_eq!(moves(
            0x0000f80006140800,
            0x8844073c38202440,
        ),
            0x6601004040480224
        ); 
    }
}
