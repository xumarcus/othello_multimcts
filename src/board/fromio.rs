use crate::*;

use std::io::Read;

impl fromIO for Board {
    fn from_io(r: &mut impl Read) -> Option<Board> {
        let mut buf = String::new();
        r.read_line(&mut buf).ok()?;
        r.read_line(&mut buf).ok()?;
        buf.parse::<Board>().ok()
    }
}
