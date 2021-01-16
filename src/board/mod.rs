mod r#impl;
mod fromstr;
mod display;
mod internals;

#[derive(Debug, Copy, Clone)]
pub struct Board {
    me: u64,
    op: u64,
    side: bool,
}
