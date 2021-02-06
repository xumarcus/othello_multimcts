mod r#impl;

use crate::*;

use rand::seq::IteratorRandom;

pub struct Algo<T: Rng> {
	algo_type: AlgoType,
	epsilon: f32,
	rng: T
}
