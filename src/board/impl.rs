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

    pub fn a_mobil_op(&self) -> u32 {
        internals::moves(self.op, self.me).count_ones()
    }

    pub fn p_mobil_op(&self) -> u32 {
        internals::potential(self.op, self.me).count_ones()
    }

    pub fn place(&self, next_move: Moves) -> Board {
        let Moves(mv) = next_move;
        let af = internals::affect(self.me, self.op, mv);
        let n_me = self.me | mv | af;
        let n_op = self.op & !af;
        debug_assert_eq!(n_me & n_op, 0);
        match Moves(internals::moves(n_op, n_me)) {
            Moves(0) => {
                let moves = Moves(internals::moves(n_me, n_op));
                Board { me: n_me, op: n_op, side: self.side, moves }
            }
            moves => Board { me: n_op, op: n_me, side: !self.side, moves }
        }
    }

    // sizeof Board too small for optimization
    pub fn place_mut(&mut self, next_move: Moves) {
        *self = self.place(next_move);
    }

    // Check if moves is valid before generation
    // For performance reasons
    pub fn place_checked(&self, next_move: Moves) -> Option<Board> {
        if self.moves.0 & next_move.0 == 0 {
            return None;
        }
        Some(self.place(next_move))
    }
}
