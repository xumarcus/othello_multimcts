use super::*;

impl FromStr for Moves {
    type Err = ParseMovesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
        	return Err(ParseMovesError::NotAscii);
        }
        match s.bytes()
            .map(|byte| byte.to_ascii_lowercase())
            .collect::<Vec<_>>() 
            .as_slice()
        {
        	[c, r] => {
        		if !(b'a'..=b'h').contains(c) {
        			return Err(ParseMovesError::InvalidColumn);
        		}
        		if !(b'1'..=b'8').contains(r) {
        			return Err(ParseMovesError::InvalidRow);
        		}
        		let n = (b'h' - c + (b'8' - r) * 8) as usize;
        		Ok(Moves(1u64 << n))
        	},
        	bytes => Err(ParseMovesError::InvalidLength(bytes.len()))
        }
    }
}
