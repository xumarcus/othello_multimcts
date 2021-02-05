mod game;
mod mcts;
mod naive;

use game::*;
use mcts::*;
use naive::*;

use othello_multimcts::*;

use std::{io, mem};

use log::*;

use rand::SeedableRng;
use rand::rngs::SmallRng;

#[macro_use]
extern crate derivative;

trait AI: std::fmt::Debug {
    fn run(&mut self) -> Failable<Moves>;
    fn update(&mut self, next_move: Moves);
}

type Failable<T> = Result<T, Box<dyn std::error::Error + 'static>>;
type Boxed = Option<Box<dyn AI>>;

#[derive(Clone, Copy, Debug, Derivative)]
#[derivative(Default)]
pub struct Config {
    board: Board,
    
    #[derivative(Default(value="1000"))]
    timeout: u64,
    threads: usize,

    #[derivative(Default(value="0.05"))]
    epsilon: f32,
    algo_type: AlgoType,
    naive: bool
}

fn main() -> Failable<()> {
    let mut game = Game::from_env()?;
    stderrlog::new().verbosity(game.verbose).init()?;
    trace!("{:?}", game);
    let mut prevs = String::new();
    while game.is_running() {
        let next_move = game.run()?;
        warn!("Move: [{}]", next_move);
        prevs.push_str(&next_move.to_string());
    }
    warn!("Prevs: [{}]", prevs);
    error!("{:?}", game.winner());
    Ok(())
}
