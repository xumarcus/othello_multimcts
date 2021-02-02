use crate::*;

impl BoardMove {
	pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
		match bytes {
			[b'p', b'a'] => Some(BoardMove(0)),
			[col, row@(b'1'..=b'8')] => {
				let r = b'8' - row;
				let c = match col {
					b'a'..=b'h' => Some(b'h' - col),
					b'A'..=b'H' => Some(b'H' - col),
					_ => None
				}?;
				let n = (c + r * 8) as usize;
				Some(BoardMove::from_index(n))
			},
			_ => None
		}
	}

	pub fn from_index(n: usize) -> Self {
		BoardMove(1u64 << n)
	}
}