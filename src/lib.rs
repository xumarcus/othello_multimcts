pub mod algo;
mod board;
mod board_move;
mod mcts;
mod node;
mod side;

use std::cmp::Ordering;
use rand::Rng;
// Ordering::Less => Black loses

use algo::*;
use board::*;
use node::*;

pub use algo::AlgoType;
pub use board::Board;
pub use board_move::BoardMove;
pub use mcts::MCTS;
pub use node::Summary;
pub use side::*;