use crate::*;

use std::error::Error;
use std::str::FromStr;

impl Error for ParseBoardError {}

impl FromStr for Board {
    type Err = ParseBoardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Board::default();
        let mut iter = s.as_bytes().chunks_exact(2);
        for chunk in &mut iter {
            let board_move = BoardMove::from_bytes(chunk)
                .ok_or(ParseBoardError::InvalidFormat)?;
            if board_move.0 != 0 {
                board = board.place(board_move)
                    .ok_or(ParseBoardError::InvalidMove(board_move))?;
            }
        }
        if iter.remainder().is_empty() {
            Ok(board)
        } else {
            Err(ParseBoardError::InvalidFormat)
        }
    }
}
