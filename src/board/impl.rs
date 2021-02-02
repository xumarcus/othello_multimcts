use super::*;

// TODO heuristic
impl Board {
    pub fn winner(&self) -> Winner {
        let ord = self.me.count_ones().cmp(&self.op.count_ones());
        match self.side {
            Side::Black => Side::from_ordering(ord),
            Side::White => Side::from_ordering(ord.reverse())
        }
    }

    pub fn actual_mobility(&self) -> u32 {
        self.moves.0.count_ones()
    }

    pub fn potential_mobility(&self) -> u32 {
        internals::potential(self.me, self.op).count_ones()
    }

    pub fn place(&self, next_move: Moves) -> Board {
        let Moves(mv) = next_move;
        let af = internals::affect(self.me, self.op, mv);
        println!("{:x} {:x} {:x} {:x}", af, self.me, self.op, mv);
        let me = self.me | mv | af;
        let op = self.op & !af;
        assert_eq!(me & op, 0);
        let moves = Moves(internals::moves(op, me));
        if moves.is_nonzero() {
            Board { op, me, side: !self.side, moves }
        } else {
            let moves = Moves(internals::moves(me, op));
            Board { me, op, side: self.side, moves }
        }
    }
}
