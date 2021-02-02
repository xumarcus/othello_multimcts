use crate::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easy() {
        assert_eq!(Board::find_moves(
            0x0000000020040000,
            0x00007f1c1c100000
        ),
            0x0044000002000800
        );
    }

    #[test]
    fn test_hard() {
        assert_eq!(Board::find_moves(
            0x0000f80006140800,
            0x8844073c38202440,
        ),
            0x6601004040480224
        ); 
    }
}

const NOT_A: u64 = 0x7f7f7f7f7f7f7f7f;
const NOT_H: u64 = 0xfefefefefefefefe;

impl Board {
    #[inline]
    pub(super) fn find_moves(me: u64, op: u64) -> u64 {
        let a = noea(me, op);
        let b = nort(me, op);
        let c = nowe(me, op);
        let d = west(me, op);
        let e = east(me, op);
        let f = sowe(me, op);
        let g = sout(me, op);
        let h = soea(me, op);
        (a | b | c | d | e | f | g | h) & !(me | op)
    }

    #[inline]
    pub(super) fn affect(me: u64, op: u64, m: u64) -> u64 {
        let a = noea(m, op);
        let b = nort(m, op);
        let c = nowe(m, op);
        let d = west(m, op);
        let e = east(m, op);
        let f = sowe(m, op);
        let g = sout(m, op);
        let h = soea(m, op);
        let i = if a & me != 0 { a } else { 0 };
        let j = if b & me != 0 { b } else { 0 };
        let k = if c & me != 0 { c } else { 0 };
        let l = if d & me != 0 { d } else { 0 };
        let m = if e & me != 0 { e } else { 0 };
        let n = if f & me != 0 { f } else { 0 };
        let o = if g & me != 0 { g } else { 0 };
        let p = if h & me != 0 { h } else { 0 };
        i | j | k | l | m | n | o | p
    }

    #[inline]
    pub(super) fn potential_moves(me: u64, op: u64) -> u64 {
        let mut fld = op | (op << 8) | (op >> 8);
        fld |= ((fld << 1) & NOT_H) | ((fld >> 1) & NOT_A);
        fld & !(me | op)
    }
}

#[inline]
fn nowe(me: u64, op: u64) -> u64 {
    let mut gen = me;
    let mut fld = 0u64;
    let pro = op & NOT_H;

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
    fld | (fld << 9) & NOT_H
}
#[inline]
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
#[inline]
fn noea(me: u64, op: u64) -> u64 {
    let mut gen = me;
    let mut fld = 0u64;
    let pro = op & NOT_A;

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
    fld | (fld << 7) & NOT_A
}
#[inline]
fn west(me: u64, op: u64) -> u64 {
    let mut gen = me;
    let mut fld = 0u64;
    let pro = op & NOT_H;

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
    fld | (fld << 1) & NOT_H
}
#[inline]
fn east(me: u64, op: u64) -> u64 {
    let mut gen = me;
    let mut fld = 0u64;
    let pro = op & NOT_A;

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
    fld | (fld >> 1) & NOT_A
}
#[inline]
fn sowe(me: u64, op: u64) -> u64 {
    let mut gen = me;
    let mut fld = 0u64;
    let pro = op & NOT_H;

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
    fld | (fld >> 7) & NOT_H
}
#[inline]
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
#[inline]
fn soea(me: u64, op: u64) -> u64 {
    let mut gen = me;
    let mut fld = 0u64;
    let pro = op & NOT_A;

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
    fld | (fld >> 9) & NOT_A
}
