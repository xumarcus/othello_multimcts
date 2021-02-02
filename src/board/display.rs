use crate::*;

use std::fmt;

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [me_p, op_p] = match self.side {
            Side::Black => ['⚫', '⚪'],
            Side::White => ['⚪', '⚫']
        };
        writeln!(f, "- ａｂｃｄｅｆｇｈ")?;
        let it = (0..64)
            .rev()
            .map(|i| 1u64 << i)
            .map(|m| match (m & self.me, m & self.op) {
                (0, 0) => Ok('⬜'),
                (_, 0) => Ok(me_p),
                (0, _) => Ok(op_p),
                _ => Err(fmt::Error)
            })
            .collect::<Result<Vec<_>, _>>()?;
        for (i, v) in it.chunks(8).enumerate() {
            let s: String = v.iter().collect();
            writeln!(f, "{} {}", i + 1, s)?;
        }
        Ok(())
    }
}

impl fmt::Display for ParseBoardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseBoardError::InvalidMove(board_move) => {
                write!(f, "InvalidMove: [{}]", board_move)
            },
            ParseBoardError::InvalidFormat => {
                write!(f, "InvalidFormat")
            }
        }
    }
}