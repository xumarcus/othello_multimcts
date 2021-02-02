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

	fn size_hint(&self) -> (usize, Option<usize>) {
		let n = self.0.count_ones() as usize;
		(n, Some(n))
	}
}
