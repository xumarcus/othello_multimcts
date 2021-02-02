use super::*;

const ROXANNE: [u64; 5] = [
    0x8100000000000081,
    0x00003c24243c0000,
    0x3c0081818181003c,
    0x003c424242423c00,
    0x42c300000000c342,
];

impl<T: Rng> Algo<T> {
	pub fn new(algo_type: AlgoType, epsilon: f32, rng: T) -> Self {
		Algo { algo_type, epsilon, rng }
	}

	pub fn next_move(&mut self, board: Board) -> Moves {
		let moves = *board.moves();
		match self.algo_type {
			AlgoType::Random => self.random_move(moves),
			AlgoType::Roxanne => {
				if self.epsilon_test() {
					self.random_move(moves)
				} else {
					ROXANNE.iter()
					.map(|mask| Moves(mask & moves.0))
					.find(Moves::is_nonzero)
					.unwrap()
				}
			},
			AlgoType::Mobility => {
				if self.epsilon_test() {
					self.random_move(moves)
				} else {
					moves.max_by_key(|next_move| {
						let new_b = board.place(*next_move);
						new_b.actual_mobility() + new_b.potential_mobility()
					}).unwrap()
				}
			}
		}
	}

	pub fn simulate(&mut self, board: Board) -> Winner {
		let mut t = board;
		while t.moves().is_nonzero() {
			t = t.place(self.next_move(t));
		}
		t.winner()
	}

	#[inline]
	fn epsilon_test(&mut self) -> bool {
		self.rng.gen::<f32>() < self.epsilon
	}

	#[inline]
	fn random_move(&mut self, moves: Moves) -> Moves {
		debug_assert!(moves.is_nonzero());
		moves.choose(&mut self.rng).unwrap()
	}
}

/*
// lsb is probably faster
#[inline]
fn nth_bit(m: u64, r: u64) -> usize {
	let a = m - ((m >> 1) & u64::MAX / 0x03);
	let b = (a & u64::MAX / 0x05) + ((a >> 2) & u64::MAX / 0x05);
	let c = (b + (b >> 4)) & u64::MAX / 0x11;
	let d = (c + (c >> 8)) & u64::MAX / 0x101;
	let mut t = (d >> 32) + (d >> 48);
	let mut s = 64;
	let mut r = r;
	s -= (u64::wrapping_sub(t, r) & 256) >> 3;
	r -= t & (u64::wrapping_sub(t, r) >> 8);
	t = (d >> (s - 16)) & 0xff;
	s -= (u64::wrapping_sub(t, r) & 256) >> 4;
	r -= t & (u64::wrapping_sub(t, r) >> 8);
	t  = (c >> (s - 8)) & 0xf;
	s -= (u64::wrapping_sub(t, r) & 256) >> 5;
	r -= t & (u64::wrapping_sub(t, r) >> 8);
	t  = (b >> (s - 4)) & 0x7;
	s -= (u64::wrapping_sub(t, r) & 256) >> 6;
	r -= t & (u64::wrapping_sub(t, r) >> 8);
	t  = (a >> (s - 2)) & 0x3;
	s -= (u64::wrapping_sub(t, r) & 256) >> 7;
	r -= t & (u64::wrapping_sub(t, r) >> 8);
	t  = (m >> (s - 1)) & 0x1;
	s -= (u64::wrapping_sub(t, r) & 256) >> 8;
	(s - 1) as usize
}
*/
