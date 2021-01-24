use crate::*;

use std::fmt;
use std::str;

impl Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lgn = (0..64).map(|i| 1u64 << i)
            .find(self.0.eq)
            .map_err(|_| fmt::Error)?;
        let bytes = [b'h' - lgn % 8, b'8' - lgn / 8];
        let utf8s = str::from_utf8(&bytes).map_err(|_| fmt::Error)?;
        f.write_str(utf8s);
        Ok(())
    }   
}
