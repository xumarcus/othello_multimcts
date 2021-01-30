use crate::*;

use std::{fmt, str};

impl fmt::Display for MoveIO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lgn = (0..64)
            .find(|i| self.m.get() == 1u64 << i)
            .ok_or(fmt::Error)?;
        let bytes = [b'h' - lgn % 8, b'8' - lgn / 8];
        let utf8s = str::from_utf8(&bytes).map_err(|_| fmt::Error)?;
        f.write_str(utf8s)
    }   
}
