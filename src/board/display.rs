use crate::*;

use std::fmt;

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let me_p = if self.side { '⚪' } else { '⚫' };
        let op_p = if self.side { '⚫' } else { '⚪' };
        f.write_str((0..64)
            .rev()
            .map(|i| 1u64 << i)
            .map(|m| match (m & self.me, m & self.op) {
                (0, 0) => Ok('⬜'),
                (_, 0) => Ok(me_p),
                (0, _) => Ok(op_p),
                _ => Err(fmt::Error)
            })
            .collect::<Result<Vec<_>, _>>()?
            .chunks(8)
            .map(|s| s.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
            .as_str()
        )
    }
}
