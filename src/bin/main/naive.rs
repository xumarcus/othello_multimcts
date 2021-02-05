use super::*;

#[derive(Debug)]
pub struct Naive {
	board: Board,
	config: Config
}

impl AI for Naive {
	fn run(&mut self) -> Failable<Moves> {
		let mut algo = Algo::new(
			self.config.algo_type,
			self.config.epsilon,
			SmallRng::from_entropy()
		);
		let next_move = algo.next_move(self.board);
		self.board.place_mut(next_move);
		Ok(next_move)
	}

	fn update(&mut self, next_move: Moves) {
		self.board.place_mut(next_move);
	}
}

impl Naive {
	pub fn new(config: Config) -> Self {
		Self { board: config.board, config }
	}
}