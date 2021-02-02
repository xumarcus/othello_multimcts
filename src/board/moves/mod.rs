mod display;
mod fromstr;
mod iterator;

use super::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Moves(pub u64);

impl Moves {
	#[inline]
	pub fn is_nonzero(&self) -> bool {
		self.0 != 0
	}
}
