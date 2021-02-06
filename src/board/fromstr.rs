use super::*;

impl FromStr for Board {
    type Err = ParseBoardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect::<Vec<_>>();
        let mut iter = chars.chunks_exact(2);
        let mut moves_it = (&mut iter).map(|chunk| chunk
            .iter()
            .collect::<String>()
            .parse::<Moves>()
            .map_err(ParseBoardError::InvalidMove))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter();
        let board = (&mut moves_it).collect::<Board>();
        if let Some(err_move) = moves_it.next() {
            let err = ParseMovesError::NotPlaceable(err_move);
            return Err(ParseBoardError::InvalidMove(err));
        }
        match iter.remainder().len() {
            0 => Ok(board),
            n => Err(ParseBoardError::InvalidLength(n))
        }
    }
}
