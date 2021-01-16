mod board;
mod mcts;

use board::Board;
use mcts::mcts;

use std::io;

fn main() -> Result<(), std::io::Error> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    io::stdin().read_line(&mut buf)?;
    if let Ok(board) = buf.parse::<Board>() {
        println!("{}", board);
    }
    Ok(())
}
