mod r#impl;

use crate::Board;
use crate::{Node, NodeInfo};

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::thread;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

pub struct MCTSInfo {
    pub infos: Vec<NodeInfo>,
    pub cnt: usize,
}

pub use r#impl::{seq, par};
