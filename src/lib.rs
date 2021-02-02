mod algo;
mod board;
mod enums;
mod node;

use getset::Getters;

#[macro_use]
extern crate derivative;
extern crate getset;
extern crate strum_macros;

pub use algo::Algo;
pub use board::Board;
pub use board::moves::Moves;
pub use enums::*;
pub use node::*;

type Winner = Option<enums::Side>;
