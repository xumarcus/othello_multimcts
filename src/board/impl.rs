use crate::board::Board;

use std::num::NonZeroU64;

impl Board {
    pub fn new(me: u64, op: u64) -> Option<Board> {
        Board::new_of(me, op, Default::default())
    }

    fn new_of(me: u64, op: u64, side: bool) -> Option<Board> {
        if me & op != 0 {
            None
        } else {
            Some(Board {
                side,
                me: NonZeroU64::new(me)?,
                op: NonZeroU64::new(op)?,
                moves: NonZeroU64::new(moves(me, op))?
            })
        }
    }

    pub fn side(&self) -> bool {
        self.side
    }

    pub fn loser(&self) -> bool {
        self.side ^ (self.me.get().count_ones() > self.op.get().count_ones())
    }

    pub fn place(&self, m: NonZeroU64) -> Result<bool, Board> {
        let me = self.me.get();
        let op = self.op.get();
        let cap = Board::DIRS
            .iter()
            .map(|f| f(m.get(), op))
            .filter(|g| g & me != 0)
            .fold(0, |a, x| a | x);
        let me: u64 = me | m.get() | cap;
        let op: u64 = op & !cap;
        let diff = Board::new_of(op, me, !self.side);
        diff.map_or(Ok(()), Err)?;
        let same = Board::new_of(me, op, self.side);
        same.map_or(Ok(()), Err)?;
        Ok(self.side ^ (me.count_ones() > op.count_ones()))
    }

    pub fn moves(&self) -> NonZeroU64 {
        self.moves
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
        let x = Board::new(0x0000000020040000, 0x00007f1c1c100000);
        assert_eq!(x.map(|x| x.moves), NonZeroU64::new(0x0044000002000800));
    }

    #[test]
    fn test_hard() {
        let x = Board::new(0x0000f80006140800, 0x8844073c38202440);
        assert_eq!(x.map(|x| x.moves), NonZeroU64::new(0x6601004040480224));
    }
}
