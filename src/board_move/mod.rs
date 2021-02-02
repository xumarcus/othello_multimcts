mod r#impl;
mod display;
mod fromstr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BoardMove(pub u64);
