use crate::*;

use std::fmt;

impl fmt::Display for MoveIO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self.0 {
            Some(m) => {
                let lg = (0..64).find(|i| m.get() == 1u64 << i).unwrap();
                let bytes = vec![b'h' - lg % 8, b'8' - lg / 8];
                String::from_utf8(bytes).map_err(|_| fmt::Error)?
            }
            _ => String::from("pa")
        };
        f.write_str(string.as_str())
    }   
}
