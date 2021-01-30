mod r#impl;

use crate::*;

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::thread;
use rand::SeedableRng;
use rand::rngs::SmallRng;

const BLOCK_SIZE: usize = 100;

#[derive(Debug)]
pub struct MCTSInfo {
    cnt: usize,
    infos: Vec<NodeInfo>
}

pub use r#impl::{seq, par};
