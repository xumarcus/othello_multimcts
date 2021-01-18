mod r#impl;
mod fromstr;
mod display;
mod internals;

use std::num::NonZeroU64;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Board {
    me: NonZeroU64,
    op: NonZeroU64,
    side: bool,
    moves: Option<NonZeroU64>
}
