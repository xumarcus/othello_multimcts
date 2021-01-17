mod board;
mod mcts;

use board::Board;
use mcts::mcts_seq;

use std::io;

fn read_board() -> Option<Board> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok()?;
    io::stdin().read_line(&mut buf).ok()?;
    buf.parse::<Board>().ok()
}

#[allow(dead_code)]
fn interactive() {
    let board = loop {
        if let Some(board) = read_board() {
            break board;
        } else {
            println!("T.T");
        }
    };
    let (cnt, moves) = mcts_seq(board, 2000);
    println!("{}", cnt);
    if let Some(next) = moves.iter().max() {
        println!("I: \n{}", board);
        println!("O: \n{}", next.board);
    }
}

#[allow(dead_code)]
fn self_play() {
    let mut board = Board::new(0x0000_0008_1000_0000, 0x0000_0010_0800_0000).unwrap();
    loop {
        println!("{}", board);
        let (cnt, moves) = mcts_seq(board, 2000);
        if let Some(next) = moves.iter().max() {
            println!("p={:.1}% n={}", next.score * 100.0, cnt);
            board = next.board;
        } else {
            println!("Done");
            break;
        }
    }
}

fn main() {
    loop { interactive() }
}
