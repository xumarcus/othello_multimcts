mod mcts;
mod board;

#[allow(unused_imports)]
use mcts::{mcts_seq, mcts_par, NextMove};
use board::Board;

use std::env;
use std::io;
use std::num::NonZeroU64;
use std::str::from_utf8;

fn run_mcts(board: Board) -> Option<(usize, NextMove)> {
    mcts_seq(board, 2000)
    .and_then(|(cnt, moves)| {
        moves.iter().max()
        .map(|next_move| (cnt, next_move.clone()))
    })
}

fn print_info((cnt,
    NextMove { board, mmask, score }): (usize, NextMove)) {
    println!("Count: {}", cnt);
    println!("Score: {:.1}%", score * 100.0);
    println!("Board:\n{}", board);
    print_move(mmask);
}

fn final_move(board: Board) -> Option<NonZeroU64> {
    let moves = board.moves().get();
    (0..64)
        .map(|i| 1u64 << i)
        .filter(|m| m & moves != 0)
        .filter_map(NonZeroU64::new)
        .filter(|m| board.place(*m).unwrap() != board.side())
        .next()
}

fn analyze() {
    loop {
        if let Some(p) = read_board().and_then(run_mcts) {
            print_info(p);
        } else {
            println!("[Error]");
        }
    }
}

fn start() {
    let mut board = Board::init();
    'outer: loop {
        let side = board.side();
        if let Some(next) = read_move()
            .filter(|m| m.get() & board.moves().get() != 0)
            .and_then(|m| board.place(m).err())
        {
            board = next;
            while side != board.side() {
                if let Some(p) = run_mcts(board) {
                    print_info(p);
                    board = p.1.board;
                } else {
                    if let Some(m) = final_move(board) {
                        print_move(m);
                    }
                    break 'outer;
                }
            }
        } else {
            println!("[Error]\n{}", board);
        }
    }
}

fn test() {
    let mut board = Board::init();
    while let Some(p) = run_mcts(board) {
        print_info(p);
        board = p.1.board;
    }
    if let Some(m) = final_move(board) {
        print_move(m);
    }
}

fn main() {
    match env::args().nth(1).as_ref().map(String::as_ref) {
        Some("analyze") => analyze(),
        Some("start")   => start(),
        Some("test")    => test(),
        _ => ()
    }
}
