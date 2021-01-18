use crate::board::Board;

use std::fmt;

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let me = self.me.get();
        let op = self.op.get();
        if let Some(ss) = (0..64)
            .rev()
            .map(|i| 1u64 << i)
            .map(|m| match (m & me, m & op) {
                (0, 0) => Some('⬜'),
                (_, 0) => Some('⚪'),
                (0, _) => Some('⚫'),
                _ => None
            })
            .collect::<Option<Vec<_>>>()
        {
            f.write_str(ss
                .chunks(8)
                .map(|s| s.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
                .as_str()
            );
        }
        Ok(())
    }
}
