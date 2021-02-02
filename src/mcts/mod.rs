mod r#impl;

use crate::*;

use std::convert::identity;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering as AtomicOrdering;
use std::time::Duration;
use std::thread;
use rand::SeedableRng;
use rand::rngs::SmallRng;

#[derive(Debug)]
pub struct MCTS {
	pub timeout: u64,
	pub threads: usize,
	pub epsilon: f32,
	pub algo_type: AlgoType
}
