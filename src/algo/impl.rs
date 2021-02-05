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
					self.random_move(ROXANNE.iter()
                        .map(|mask| Moves(mask & moves.0))
                        .find(Moves::is_nonzero)
                        .expect("Masks entirely cover")
                    )
				}
			},
			AlgoType::Mobility => {
				if self.epsilon_test() {
					self.random_move(moves)
				} else {
                    // Imperative cuz of early return
                    let mut max_v = 0;
                    let mut max_m = Moves(0);
                    for next_move in moves {
                        let new_b = board.place(next_move);
                        if new_b.side() == board.side() {
                            return next_move;
                        }
                        let mobil = new_b.a_mobil_op() + new_b.p_mobil_op();
                        if mobil >= max_v {
                            max_v = mobil;
                            max_m = next_move;
                        }
                    }
                    max_m
				}
			}
		}
	}

	pub fn simulate(&mut self, board: Board) -> Winner {
		let mut t = board;
		while t.moves().is_nonzero() {
			t.place_mut(self.next_move(t));
		}
		t.winner()
	}

	#[inline]
	fn epsilon_test(&mut self) -> bool {
		self.rng.gen::<f32>() < self.epsilon
	}

	#[inline]
	fn random_move(&mut self, moves: Moves) -> Moves {
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
