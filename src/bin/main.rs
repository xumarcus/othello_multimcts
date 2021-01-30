use otrs::*;

use std::io;
use std::num::NonZeroU64;

fn next_move(moves: NonZeroU64) -> NonZeroU64 {
    let stdin = io::stdin();
    loop {
        println!("Next?");
        if let Some(MoveIO { m }) = MoveIO::from_io(&mut stdin.lock()) {
            if m.get() & moves.get() != 0 {
                break m;
            }
        }
    }
}

fn main() {
    let mut board = Board::new();
    while let Some(moves) = board.moves() {
        println!("{}", board);
        let next_move = if board.side() {
            let mcts_info = seq(board, 1000).unwrap();
            println!("Eval: {}", mcts_info.cnt());
            mcts_info.best(true).unwrap()
        } else {
            next_move(moves)
        };
        println!("Move: [{}]", MoveIO { m: next_move });
        if let Some(new_board) = board.place(next_move) {
            board = new_board;
        } else {
            break;
        }
    }
}
