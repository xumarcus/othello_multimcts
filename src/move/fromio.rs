use crate::*;

use std::io::Read;

impl FromIO for Move {
    fn from_io(r: &mut impl Read) -> Option<Move> {
        let mut buf = String::new();
        r.read_line(&mut buf).ok()?;
        match buf.trim().as_bytes() {
            [col@(b'a'..=b'h'), row@(b'1'..=b'8')] => {
                let n = (b'h' - col) + (b'8' - row) * 8;
                Move(NonZeroU64::new(1u64 << n))
            },
            _ => None
        }
    }
}
