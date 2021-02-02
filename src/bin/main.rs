use otrs::*;

use std::convert::Infallible;
use std::error::Error;
use std::str::FromStr;
use std::io;
use clap::{Arg, App};

const ALGO_TYPES: [&'static str; 3] = ["random", "roxanne", "mobility"];
const LOG_LEVELS: [&'static str; 5] = ["silent", "minimal", "interact", "info", "debug"];

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum LogLevel {
    Silent,
    Minimal,
    Interact,
    Info,
    Debug
}

impl FromStr for LogLevel {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "silent" => LogLevel::Silent,
            "minimal" => LogLevel::Minimal,
            "interact" => LogLevel::Interact,
            "info" => LogLevel::Info,
            "debug" => LogLevel::Debug,
            _ => panic!()
        })
    }
}

type Res<T> = Result<T, Box<dyn Error + 'static>>;

fn main() -> Res<()> {
    let matches = App::new("Othello-MultiMCTS")
        .version("0.0.3.0")
        .author("Marcus Xu <xumarcus.sg@gmail.com>")
        .about("Multithreaded MCTS-based Othello AI")
        .long_about("These AI options are supported:\n\
                    [none]: from stdin (recommend log-level at least interact)\n\
                    [random]: simulate moves randomly without heuristic\n\
                    [roxanne]: p=0.88±0.11 against [random]\n\
                    See Archer, R. (2007). Analysis of Monte Carlo Techniques in Othello.")
        .arg(Arg::new("timeout")
            .short('t')
            .long("timeout")
            .default_value("1000")
            .about("Set how long the AI should run"))
        .arg(Arg::new("threads")
            .short('p')
            .long("threads")
            .default_value("0")
            .about("Run multithreaded with x number of threads")
            .long_about("This feature is experimental. Implementing WU-UCT for parallelism. \
                        Speedup depends on target architecture and algorithm chosen. \
                        Generally, slower simulation benefit more from more threads."))
        .arg(Arg::new("epsilon")
            .short('e')
            .long("epsilon")
            .default_value("0.05")
            .about("Set parameter for epsilon-greedy simulation"))
        .arg(Arg::new("log-level")
            .short('l')
            .alias("log")
            .long("log-level")
            .default_value("info")
            .possible_values(&LOG_LEVELS)
            .about("Set logging level"))
        .arg(Arg::new("from-sequence")
            .short('s')
            .alias("sequence")
            .long("from-sequence")
            .takes_value(true)
            .about("Start game from sequence of moves"))
        .arg(Arg::new("black-player")
            .short('b')
            .alias("black")
            .long("black-player")
            .takes_value(true)
            .possible_values(&ALGO_TYPES)
            .about("Set AI for Black."))
        .arg(Arg::new("white-player")
            .short('w')
            .alias("white")
            .long("white-player")
            .default_value("roxanne")
            .possible_values(&ALGO_TYPES)
            .about("Set AI for White."))
        .get_matches();

    let timeout = matches.value_of("timeout")
        .unwrap().parse()?;
    let threads = matches.value_of("threads")
        .unwrap().parse()?;
    let epsilon = matches.value_of("epsilon")
        .unwrap().parse()?;
    let log_level = matches.value_of("log-level")
        .unwrap().parse::<LogLevel>()?;
    let make_player = |s| matches.value_of(s).map(|s| {
        MCTS {
            timeout,
            threads,
            epsilon,
            // TODO fromStr
            algo_type: match s {
                "random" => AlgoType::Random,
                "roxanne" => AlgoType::Roxanne,
                "mobility" => AlgoType::Mobility,
                _ => AlgoType::default()
            } 
        }
    });
    let players = [
        (make_player("black-player"), Side::Black),
        (make_player("white-player"), Side::White)
    ];
    if log_level >= LogLevel::Debug {
        println!("{:#?}", players);
    }
    let mut board = match matches.value_of("from-sequence") {
        Some(s) => s.parse()?,
        None => Board::default()
    };
    let mut cycle = players.iter().cycle();
    if board.side() != Side::default() {
        cycle.next();
    }
    let mut prev_str = String::new();
    let mut prev_passed = false;
    for (player, side) in cycle {
        if log_level >= LogLevel::Interact {
            println!("----------------\nBoard:\n{}", board);
            println!("Moves: [{}]", board.moves());
        }
        if log_level >= LogLevel::Info {
            println!("Prevs: [{}]", prev_str);
            println!("Current side: {}", board.side());
            println!("Winning side: {:?}", Side::winner(board.ordering()));
        }
        let board_move = match player {
            Some(ai) => {
                Some(())    // bool_to_option
                .filter(|_| *side == board.side())
                .and_then(|_| ai.run(board))
                .map(|summary| {
                    if log_level >= LogLevel::Info {
                        println!("Count: {}", summary.count);
                        println!("Prob: {:.1}%", summary.score * 100.0);
                    }
                    summary.next_move
                })
                .unwrap_or(BoardMove(0))
            },
            None => loop {
                if log_level >= LogLevel::Interact {
                    if *side == board.side() {
                        println!("Next?");
                    } else {
                        println!("Pass with [pa]!");
                    }
                }
                let mut buf = String::new();
                io::stdin().read_line(&mut buf)?;
                match buf.trim().parse() {
                    Ok(board_move) => break board_move,
                    Err(err) => if log_level >= LogLevel::Interact {
                        println!("{}", err);
                    }
                }
            }
        };
        prev_str.push_str(&board_move.to_string());
        if log_level >= LogLevel::Minimal {
            println!("Move: [{}]", board_move);
        }
        match board_move {
            BoardMove(0) => {
                if prev_passed {
                    break;
                }
                prev_passed = true;
            },
            board_move => {
                board = board.place(board_move).unwrap();
                prev_passed = false;
            }
        }
    }
    if log_level >= LogLevel::Minimal {
        println!("Prevs: [{}]", prev_str);
    }
    match Side::winner(board.ordering()) {
        Some(side) => println!("{} wins", side),
        _ => println!("Draw")
    };
    Ok(())
}