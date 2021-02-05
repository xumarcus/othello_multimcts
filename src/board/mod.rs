mod r#impl;
mod display;
mod fromiterator;
mod fromstr;
mod internals;
pub mod moves;

use crate::*;

use std::fmt;
use std::iter::{FromIterator, IntoIterator, Iterator, ExactSizeIterator};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Derivative, Getters, PartialEq, Eq)]
#[derivative(Default)]
pub struct Board {

	#[derivative(Default(value = "0x0000_0008_1000_0000"))]
    me: u64,

    #[derivative(Default(value = "0x0000_0010_0800_0000"))]
    op: u64,

    #[getset(get = "pub")]
    side: Side,

    #[derivative(Default(value = "Moves(0x0000_1020_0408_0000)"))]
    #[getset(get = "pub")]
    moves: Moves,
}
