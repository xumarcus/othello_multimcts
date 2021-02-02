use super::*;

use std::cmp::Ordering;
use std::ops::Not;

#[derive(Derivative)]
#[derivative(Default)]
#[derive(Clone, Copy, Debug, Display, EnumString, PartialEq, Eq)]
pub enum Side {
	#[derivative(Default)]
	Black,
	White
}

impl Not for Side {
	type Output = Self;
	fn not(self) -> Self::Output {
		match self {
			Side::Black => Side::White,
			Side::White => Side::Black
		}
	}
}

impl Side {
	pub fn from_ordering(ord: Ordering) -> Option<Self> /* aka Winner */ {
		match ord {
			Ordering::Less => Some(Side::White),
			Ordering::Equal => None,
			Ordering::Greater => Some(Side::Black)
		}
	}
}
