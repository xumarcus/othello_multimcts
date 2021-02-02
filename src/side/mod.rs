mod r#impl;
mod default;
mod display;
mod not;

use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Side {
	Black,
	White
}

