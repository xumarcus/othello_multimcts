mod r#impl;
mod app;

use othello_multimcts::*;
use app::app;

use std::error::Error;
use std::io;

use clap::{App, Arg};
use strum::VariantNames;

#[derive(Debug)]
pub struct MCTS {
    timeout: u64,
    threads: usize,
    epsilon: f32,
    algo_type: AlgoType
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let matches = app().get_matches();

    let timeout = matches.value_of("timeout")
        .unwrap().parse()?;
    let threads = matches.value_of("threads")
        .unwrap().parse()?;
    let epsilon = matches.value_of("epsilon")
        .unwrap().parse()?;
    let log_level = matches.value_of("log-level")
        .unwrap().parse::<LogLevel>()?;
    let make_player = |s| matches.value_of(s)
        .and_then(|t| t.parse::<AlgoType>().ok())
        .map(|algo_type| MCTS::new(timeout, threads, epsilon, algo_type));
    let players = [
        (make_player("black-player"), Side::Black),
        (make_player("white-player"), Side::White)
    ];
    if log_level >= LogLevel::Debug {
        println!("{:#?}", players);
    }

    let mut prev_str = String::new();
    let mut prev_passed = false;
    let mut board = match matches.value_of("from-sequence") {
        Some(s) => s.parse()?,
        None => Board::default()
    };
    let mut cycle = players.iter().cycle();
    if *board.side() != Side::default() {
        cycle.next();
    }
    
    for (player, side) in cycle {
        if log_level >= LogLevel::Interact {
            println!("----------------\nBoard:\n{}", board);
            println!("Moves: [{}]", board.moves());
        }
        if log_level >= LogLevel::Info {
            println!("Prevs: [{}]", prev_str);
            println!("Current side: {}", side);
            println!("Winning side: {:?}", board.winner());
        }
        if log_level >= LogLevel::Debug {
            println!("{:#?}", board);
        }
        let moves = match player {
            Some(ai) => {
                if *side == *board.side() {
                    ai.run(board).map(|best| {
                        if log_level >= LogLevel::Info {
                            println!("Count: {}", best.n());
                            println!("Prob: {:.1}%", best.avg() * 100.0);
                        }
                        *best.next_move()
                    }).unwrap_or(Moves(0))
                } else {
                    Moves(0)
                }
            },
            None => loop {
                if log_level >= LogLevel::Interact {
                    if *side == *board.side() {
                        println!("Next?");
                    } else {
                        println!("Pass with [pa]!");
                    }
                }
                let mut buf = String::new();
                io::stdin().read_line(&mut buf)?;
                match buf.trim().parse() {
                    Ok(moves) => break moves,
                    Err(err) => if log_level >= LogLevel::Interact {
                        println!("{}", err);
                    }
                }
            }
        };
        prev_str.push_str(&moves.to_string());
        if log_level >= LogLevel::Minimal {
            println!("Move: [{}]", moves);
        }
        match moves {
            Moves(0) => {
                if prev_passed {
                    break;
                }
                prev_passed = true;
            },
            moves => {
                board = board.place(moves);
                prev_passed = false;
            }
        }
    }
    if log_level >= LogLevel::Minimal {
        println!("Prevs: [{}]", prev_str);
    }
    println!("Winner: {:?}", board.winner());
    Ok(())
}
