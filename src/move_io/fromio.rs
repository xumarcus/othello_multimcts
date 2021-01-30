use crate::*;
use crate::MoveIO;

use std::io::{Read, BufRead};

impl FromIO for MoveIO {
    fn from_io<R: Read + BufRead>(r: &mut R) -> Option<MoveIO> {
        let mut buf = String::new();
        r.read_line(&mut buf).ok()?;
        match buf.trim().as_bytes() {
            [col@(b'a'..=b'h'), row@(b'1'..=b'8')] => {
                let n = (b'h' - col) + (b'8' - row) * 8;
                let m = NonZeroU64::new(1u64 << (n as usize));
                Some(MoveIO(m))
            },
            [b'p', b'a'] => Some(MoveIO(None)),
            _ => None
        }
    }
}
