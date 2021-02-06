use super::*;

use clap::{App, Arg};
use strum::VariantNames;

impl Game {
    pub fn from_env() -> Failable<Game> {
        let matches = App::new("Othello-MultiMCTS")
            .version(env!("CARGO_PKG_VERSION"))
            .author("Marcus Xu <xumarcus.sg@gmail.com>")
            .about("Multithreaded MCTS-based Othello AI")
            .long_about("These AI options are supported:\n\
[none]: from stdin (recommend log-level at least interact)\n\
[random]: simulate move randomly without heuristic\n\
[roxanne]: p=0.88±0.11 against [random]\n\
See Archer, R. (2007). Analysis of Monte Carlo Techniques in Othello.")
            .arg(Arg::new("from-sequence")
                .short('s')
                .alias("sequence")
                .long("from-sequence")
                .takes_value(true)
                .about("Start game from sequence of moves"))
            .arg(Arg::new("verbose")
                .short('v')
                .long("verbose")
                .required_unless_present_all(&["black", "white"])
                .multiple_occurrences(true))
            .arg(Arg::new("timeout")
                .short('t')
                .long("timeout")
                .default_value(&Config::default().timeout.to_string())
                .multiple(true)
                .max_values(2)
                .about("Set how long the AI should run"))
            .arg(Arg::new("threads")
                .short('p')
                .long("threads")
                .default_value(&Config::default().threads.to_string())
                .multiple(true)
                .max_values(2)
                .about("Run multithreaded with x number of threads")
                .long_about("[Experimental] Implementing WU-UCT for parallelism. \
Speedup depends on target architecture and algorithm chosen. \
Generally, slower simulation benefit more from more threads."))
            .arg(Arg::new("epsilon")
                .short('e')
                .long("epsilon")
                .default_value(&Config::default().epsilon.to_string())
                .multiple(true)
                .max_values(2)
                .about("Set parameter for epsilon-greedy simulation"))
            .arg(Arg::new("algo_type")
                .short('a')
                .long("algo-type")
                .default_value(&Config::default().algo_type.to_string())
                .multiple(true)
                .max_values(2)
                .possible_values(&AlgoType::VARIANTS)
                .about("Set AI Algorithm"))
            .arg(Arg::new("naive")
                .short('n')
                .long("naive")
                .default_value(&Config::default().naive.to_string())
                .multiple(true)
                .max_values(2)
                .about("Disables MCTS for baseline"))
            .arg(Arg::new("black")
                .short('b')
                .long("black")
                .about("AI will play Black"))
            .arg(Arg::new("white")
                .short('w')
                .long("white")
                .about("AI will play White."))
            .get_matches();

        let board = matches.value_of("from-sequence")
            .map(str::parse::<Board>)
            .transpose()?
            .unwrap_or(Board::default());

        macro_rules! params {
            (mut $x: ident: $t: ty) => {
                let v = match matches.values_of(stringify!($x)) {
                    None => Vec::new(),
                    Some(it) => it.map(str::parse::<$t>)
                        .collect::<Result<Vec<_>, _>>()?
                };
                let mut $x = v.into_iter();
            };
        }
        params!(mut timeout: u64);
        params!(mut threads: usize);
        params!(mut epsilon: f32);
        params!(mut algo_type: AlgoType);
        params!(mut naive: bool);

        let mut generate_config = || {
            let mut config = Config::default();
            macro_rules! set_config {
                ($params: ident) => {
                    if let Some(param) = $params.next() {
                        config.$params = param;
                    }
                };
            }
            set_config!(timeout);
            set_config!(threads);
            set_config!(epsilon);
            set_config!(algo_type);
            set_config!(naive);
            Config { board, ..config }
        };
        

        Ok(Game {
            board,
            black: {
                if matches.is_present("black") {
                    let config = generate_config();
                    let x: Box<dyn AI> = match config.naive {
                        true => Box::new(Naive::new(config)),
                        _ => Box::new(MCTS::new(config))
                    };
                    Some(x)
                } else {
                    None
                }
            },
            white: {
                if matches.is_present("white") {
                    let config = generate_config();
                    let x: Box<dyn AI> = match config.naive {
                        true => Box::new(Naive::new(config)),
                        _ => Box::new(MCTS::new(config))
                    };
                    Some(x)
                } else {
                    None
                }
            },
            verbose: matches.occurrences_of("verbose") as usize,
            ..Game::default()
        })
    }
}
