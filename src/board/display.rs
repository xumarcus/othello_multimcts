use crate::*;

use std::fmt;

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let me = self.me.get();
        let op = self.op.get();
        let ss = (0..64)
            .rev()
            .map(|i| 1u64 << i)
            .map(|m| match (m & me, m & op) {
                (0, 0) => Ok('⬜'),
                (_, 0) => Ok('⚪'),
                (0, _) => Ok('⚫'),
                _ => Err(fmt::Error)
            })
            .collect::<Result<_, _>>()?
            .chunks(8)
            .map(|s| s.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        f.write_str(ss.as_str());
        Ok(())
    }
}
