use otrs::*;

use std::io;
use std::num::NonZeroU64;

const AI: bool = true;
const TIMEOUT: u64 = 1000;

fn next_move_from_input(moves: Option<NonZeroU64>) -> Option<NonZeroU64> {
    let stdin = io::stdin();
    loop {
        println!("Next?");
        match MoveIO::from_io(&mut stdin.lock()) {
            Some(MoveIO(m)) => match moves {
                Some(moves) => {
                    if let Some(m_nz) = m {
                        if m_nz.get() & moves.get() != 0 {
                            return m;
                        } else {
                            println!("Occupied");
                        }
                    } else {
                        println!("Cannot pass");
                    }
                },
                _ => if m.is_none() {
                    return m;
                } else {
                    println!("Pass with [pa]!");
                }
            }
            _ => println!("Invalid!")
        }
    }
}

fn main() {
    let mut board = Board::new();
    let mut cur = !AI;
    let mut prev_pass = false;
    loop {
        println!("{}", board);
        let next_move = if cur == AI {
            if cur == board.side() {
                seq(board, TIMEOUT)
                .and_then(|mcts_info| {
                    println!("Eval: {}", mcts_info.cnt());
                    mcts_info.best(AI)
                })
            } else {
                None
            }
        } else {
            next_move_from_input(board.moves()
                .filter(|_| cur == board.side()))
        };
        println!("Move: [{}]", MoveIO(next_move));
        if let Some(next) = next_move {
            board = board.place(next).unwrap();
            prev_pass = false;
        } else if prev_pass {
            break;
        } else {
            prev_pass = true;
        }
        cur = !cur;
    }
}
