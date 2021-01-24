mod r#impl;

use crate::*;

pub struct NodeInfo {
    pub board: Board,
    pub cur: Option<NonZeroU64>,
    pub data: Result<bool, (usize, usize)>
    pub data: Result<bool, >
}
