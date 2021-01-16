use crate::board::Board;

impl Board {
    const NOT_A: u64 = 0x7f7f7f7f7f7f7f7f;
    const NOT_H: u64 = 0xfefefefefefefefe;
    pub(super) const DIRS: [fn(u64, u64) -> u64; 8] = [
        Board::noea,
        Board::nort,
        Board::nowe,
        Board::west,
        Board::east,
        Board::sowe,
        Board::sout,
        Board::soea,
    ];

    fn nowe(me: u64, op: u64) -> u64 {
        let mut gen = me;
        let mut fld = 0u64;
        let pro = op & Board::NOT_H;

        gen = (gen << 9) & pro;
        fld |= gen;
        gen = (gen << 9) & pro;
        fld |= gen;
        gen = (gen << 9) & pro;
        fld |= gen;
        gen = (gen << 9) & pro;
        fld |= gen;
        gen = (gen << 9) & pro;
        fld |= gen;
        gen = (gen << 9) & pro;
        fld |= gen;
        fld | (fld << 9) & Board::NOT_H
    }

    fn nort(me: u64, op: u64) -> u64 {
        let mut gen = me;
        let mut fld = 0u64;
        let pro = op;

        gen = (gen << 8) & pro;
        fld |= gen;
        gen = (gen << 8) & pro;
        fld |= gen;
        gen = (gen << 8) & pro;
        fld |= gen;
        gen = (gen << 8) & pro;
        fld |= gen;
        gen = (gen << 8) & pro;
        fld |= gen;
        gen = (gen << 8) & pro;
        fld |= gen;
        fld | fld << 8
    }

    fn noea(me: u64, op: u64) -> u64 {
        let mut gen = me;
        let mut fld = 0u64;
        let pro = op & Board::NOT_A;

        gen = (gen << 7) & pro;
        fld |= gen;
        gen = (gen << 7) & pro;
        fld |= gen;
        gen = (gen << 7) & pro;
        fld |= gen;
        gen = (gen << 7) & pro;
        fld |= gen;
        gen = (gen << 7) & pro;
        fld |= gen;
        gen = (gen << 7) & pro;
        fld |= gen;
        fld | (fld << 7) & Board::NOT_A
    }

    fn west(me: u64, op: u64) -> u64 {
        let mut gen = me;
        let mut fld = 0u64;
        let pro = op & Board::NOT_H;

        gen = (gen << 1) & pro;
        fld |= gen;
        gen = (gen << 1) & pro;
        fld |= gen;
        gen = (gen << 1) & pro;
        fld |= gen;
        gen = (gen << 1) & pro;
        fld |= gen;
        gen = (gen << 1) & pro;
        fld |= gen;
        gen = (gen << 1) & pro;
        fld |= gen;
        fld | (fld << 1) & Board::NOT_H
    }

    fn east(me: u64, op: u64) -> u64 {
        let mut gen = me;
        let mut fld = 0u64;
        let pro = op & Board::NOT_A;

        gen = (gen >> 1) & pro;
        fld |= gen;
        gen = (gen >> 1) & pro;
        fld |= gen;
        gen = (gen >> 1) & pro;
        fld |= gen;
        gen = (gen >> 1) & pro;
        fld |= gen;
        gen = (gen >> 1) & pro;
        fld |= gen;
        gen = (gen >> 1) & pro;
        fld |= gen;
        fld | (fld >> 1) & Board::NOT_A
    }

    fn sowe(me: u64, op: u64) -> u64 {
        let mut gen = me;
        let mut fld = 0u64;
        let pro = op & Board::NOT_H;

        gen = (gen >> 7) & pro;
        fld |= gen;
        gen = (gen >> 7) & pro;
        fld |= gen;
        gen = (gen >> 7) & pro;
        fld |= gen;
        gen = (gen >> 7) & pro;
        fld |= gen;
        gen = (gen >> 7) & pro;
        fld |= gen;
        gen = (gen >> 7) & pro;
        fld |= gen;
        fld | (fld >> 7) & Board::NOT_H
    }

    fn sout(me: u64, op: u64) -> u64 {
        let mut gen = me;
        let mut fld = 0u64;
        let pro = op;

        gen = (gen >> 8) & pro;
        fld |= gen;
        gen = (gen >> 8) & pro;
        fld |= gen;
        gen = (gen >> 8) & pro;
        fld |= gen;
        gen = (gen >> 8) & pro;
        fld |= gen;
        gen = (gen >> 8) & pro;
        fld |= gen;
        gen = (gen >> 8) & pro;
        fld |= gen;
        fld | fld >> 8
    }

    fn soea(me: u64, op: u64) -> u64 {
        let mut gen = me;
        let mut fld = 0u64;
        let pro = op & Board::NOT_A;

        gen = (gen >> 9) & pro;
        fld |= gen;
        gen = (gen >> 9) & pro;
        fld |= gen;
        gen = (gen >> 9) & pro;
        fld |= gen;
        gen = (gen >> 9) & pro;
        fld |= gen;
        gen = (gen >> 9) & pro;
        fld |= gen;
        gen = (gen >> 9) & pro;
        fld |= gen;
        fld | (fld >> 9) & Board::NOT_A
    }
}
