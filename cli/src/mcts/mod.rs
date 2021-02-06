mod r#impl;

use super::*;

use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering as AtomicOrdering;
use std::time::Duration;
use std::thread;

#[derive(Debug)]
pub struct MCTS {
    root: Node,
    config: Config
}
