use crate::*;

use std::str::FromStr;

impl FromStr for BoardMove {
    type Err = ParseBoardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BoardMove::from_bytes(s.as_bytes())
        	.ok_or(ParseBoardError::InvalidFormat)
    }
}