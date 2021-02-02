use super::*;

impl fmt::Display for Moves {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            0 => write!(f, "pa"),
            m => {
                let s: String = (0..64)
                    .filter(|i| m & 1 << i != 0)
                    .map(|i| String::from_utf8(vec![b'h' - i % 8, b'8' - i / 8]))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|_| fmt::Error)?
                    .join(", ");
                write!(f, "{}", s)
            }
        }
    }   
}
