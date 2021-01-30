mod r#impl;
mod fromstr;
mod display;
mod internals;

use std::num::NonZeroU64;

#[derive(Debug, Copy, Clone)]
pub struct Board {
    me: u64,
    op: u64,
    side: bool,
    moves: Option<NonZeroU64>
}
