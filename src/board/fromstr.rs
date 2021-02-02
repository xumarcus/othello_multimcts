use super::*;

impl FromStr for Board {
    type Err = ParseBoardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Board::default();
        let chars = s.chars().collect::<Vec<_>>();
        let mut iter = chars.chunks_exact(2);
        for chunk in &mut iter {
            let moves = chunk.iter()
                .collect::<String>()
                .parse::<Moves>()
                .map_err(ParseBoardError::InvalidFormat)?;
            if moves.0 & board.moves().0 != 0 {
                board = board.place(moves)
            } else {
                return Err(ParseBoardError::InvalidMove(moves));
            }
        }
        match iter.remainder().len() {
            0 => Ok(board),
            n => Err(ParseBoardError::InvalidLength(n))
        }
    }
}
