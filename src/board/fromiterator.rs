use super::*;

impl FromIterator<Moves> for Board {
	fn from_iter<I: IntoIterator<Item=Moves>>(iter: I) -> Self {
		let mut board = Self::default();
        for moves in iter {
            if moves.0 & board.moves().0 != 0 {
                board = board.place(moves);
            } else {
                break;
            }
        }
		board
	}
}
