mod board;
mod node;
mod node_info;

use std::num::NonZeroU64;

use board::Board;
use node::Node;
use node_info::NodeInfo;

type StateAction = (Board, NonZeroU64);

trait FromIO where Self: Sized {
    fn from_io(r: &mut impl std::io::Read) -> Option<Self>;
}
