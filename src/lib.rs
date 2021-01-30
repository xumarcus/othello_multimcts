mod board;
mod mcts;
mod move_io;
mod node;

use std::num::NonZeroU64;

pub trait FromIO where Self: Sized {
    fn from_io<R: std::io::Read + std::io::BufRead>(r: &mut R) -> Option<Self>;
}

pub use board::Board;
pub use mcts::{seq, par, MCTSInfo};
pub use move_io::MoveIO;
pub use node::{Node, NodeInfo, simulate};
