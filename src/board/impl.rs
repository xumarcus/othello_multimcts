use crate::*;
use super::*;

impl Board {
    pub fn new(me: u64, op: u64, side: Side) -> Option<Board> {
        if me & op != 0 {
            return None;
        }
        let moves = BoardMove(Board::find_moves(me, op));
        Some(Board { me, op, side, moves })
    }

    pub fn side(&self) -> Side {
        self.side
    }

    pub fn moves(&self) -> BoardMove {
        self.moves
    }

    pub fn ordering(&self) -> Ordering {
        let ord = self.me.count_ones().cmp(&self.op.count_ones());
        match self.side {
            Side::Black => ord,
            Side::White => ord.reverse()
        }
    }

    pub fn actual_mobility(&self) -> u32 {
        self.moves.0.count_ones()
    }

    pub fn potential_mobility(&self) -> u32 {
        Board::potential_moves(self.me, self.op).count_ones()
    }

    pub fn place(&self, m: BoardMove) -> Option<Board> {
        let BoardMove(mu) = m;
        let af = Board::affect(self.me, self.op, mu);
        let me = self.me | mu | af;
        let op = self.op & !af;
        Board::new(op, me, !self.side)
        .filter(|board| board.moves().0 != 0)
        .or_else(|| Board::new(me, op, self.side))
    }
}
