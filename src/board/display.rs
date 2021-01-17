use crate::board::Board;

use std::fmt;
use std::str;

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ov: Option<Vec<u8>> = (0..64)
            .rev()
            .map(|i| {
                let m = 1u64 << i;
                match (m & self.me.get(), m & self.op.get()) {
                    (0, 0) => Some(b'.'),
                    (_, 0) => Some(b'O'),
                    (0, _) => Some(b'X'),
                    _ => None,
                }
            })
            .collect();
        match ov {
            None => Err(fmt::Error),
            Some(v) => {
                let strs: Vec<&str> = v
                    .chunks(8)
                    .map(str::from_utf8)
                    .map(Result::unwrap)
                    .collect();
                f.write_str(&strs.join("\n"))
            }
        }
    }
}
