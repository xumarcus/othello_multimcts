mod r#impl;

use super::*;

#[derive(Debug)]
pub struct Naive {
	board: Board,
	config: Config
}