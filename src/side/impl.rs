use super::*;

impl Side {
	pub fn winner(ordering: Ordering) -> Option<Self> {
		match ordering {
			Ordering::Less => Some(Side::White),
			Ordering::Greater => Some(Side::Black),
			_ => None
		}
	}

	pub fn loser(ordering: Ordering) -> Option<Self> {
		match ordering {
			Ordering::Less => Some(Side::Black),
			Ordering::Greater => Some(Side::White),
			_ => None
		}
	}
}