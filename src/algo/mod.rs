mod r#impl;

use crate::*;

#[derive(Debug, Copy, Clone)]
pub enum AlgoType {
	Random,
	Roxanne,
	Mobility,
}

pub struct Algo<T: Rng> {
	algo_type: AlgoType,
	epsilon: f32,
	rng: T
}