use crate::*;

use std::ops::Not;

impl Not for Side {
	type Output = Self;
	fn not(self) -> Self::Output {
		match self {
			Side::Black => Side::White,
			Side::White => Side::Black
		}
	}
}