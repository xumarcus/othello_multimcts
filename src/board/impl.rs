use crate::board::Board;

use std::num::NonZeroU64;

impl Board {
    pub fn new() -> Board {
        let me = 0x0000_0008_1000_0000;
        let op = 0x0000_0010_0800_0000;
        let moves = NonZeroU64::new(moves(me, op));
        unsafe {
            let me = NonZeroU64::new_unchecked(me);
            let op = NonZeroU64::new_unchecked(op);
            let side = Default::default();
            Board { me, op, side, moves }
        }
    }

    fn make(me: u64, op: u64, side: bool) -> Option<Board> {
        let moves = NonZeroU64::new(moves(me, op));
        let me = NonZeroU64::new(me)?;
        let op = NonZeroU64::new(op)?;
        Some(Board { me, op, side, moves })
    }

    pub fn side(&self) -> bool {
        self.side
    }

    pub fn moves(&self) -> Option<NonZeroU64> {
        self.moves
    }

    pub fn loser(&self) -> bool {
        let me = self.me.get();
        let op = self.op.get();

        self.side() ^ (me.count_ones() > op.count_ones())
    }

    pub fn place(&self, m: Option<NonZeroU64>) -> Option<Board> {
        let me = self.me.get();
        let op = self.op.get();
        let m = m.map(NonZeroU64::get)?;
        let c = Board::DIRS
            .iter()
            .map(|f| f(m, op))
            .filter(|g| g & me != 0)
            .fold(0, |a, x| a | x);
        let me = me | (m | c);
        let op = op & !c;
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
