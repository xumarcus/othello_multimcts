use crate::board::Board;

use std::num::ParseIntError;
use std::str::FromStr;

pub enum ParseBoardError {
    BadHex,
    BadSet,
    BadFormat
}

impl From<ParseIntError> for ParseBoardError {
    fn from(_: ParseIntError) -> ParseBoardError {
        ParseBoardError::BadHex
    }
}

impl FromStr for Board {
    type Err = ParseBoardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sp = s.split('\n');
        let me = sp.next().ok_or(ParseBoardError::BadFormat)?;
        let me = u64::from_str_radix(me, 16)?;
        let op = sp.next().ok_or(ParseBoardError::BadFormat)?;
        let op = u64::from_str_radix(op, 16)?;
        Board::make(me, op, bool::default()).ok_or(ParseBoardError::BadSet)
    }
}
