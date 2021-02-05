use super::*;

impl FromIterator<Moves> for Board {
	fn from_iter<I: IntoIterator<Item=Moves>>(iter: I) -> Self {
		let mut board = Self::default();
        for moves in iter {
            match board.place_checked(moves) {
                Some(new_b) => board = new_b,
                None => break
            }    
        }
		board
	}
}
