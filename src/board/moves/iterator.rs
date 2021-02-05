use super::*;

impl Iterator for Moves {
	type Item = Moves;

	fn next(&mut self) -> Option<Self::Item> {
		if self.0 == 0 {
			return None;
		}
		let x = self.0 & (!self.0 + 1);
		self.0 -= x;
		Some(Moves(x))
	}
}

impl ExactSizeIterator for Moves {
    fn len(&self) -> usize {
        self.0.count_ones() as usize
    }
}
