use super::*;

impl Game {
    fn check(board: &Board, moves: Moves) -> Failable<Moves> {
        match board.place_checked(moves) {
            None => Err(ParseMovesError::NotPlaceable(moves).into()),
            _ => Ok(moves)
        }
    }

    fn from_stdin(board: &Board) -> Failable<Failable<Moves>> {
        info!("Next?");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        Ok(buf.trim().parse::<Moves>()
           .map_err(ParseMovesError::into)
           .and_then(|moves| Game::check(board, moves)))
    }

    pub fn winner(&self) -> Option<Side> {
        self.board.winner()
    }

    pub fn is_running(&self) -> bool {
        let board = &self.board;
        info!("--------");
        info!("Board:\n{}", board);
        info!("Moves: [{}]", board.moves());
        debug!("Current: {}", self.side);
        debug!("Winning: {:?}", board.winner());
        trace!("{:?}", board);
        board.moves().is_nonzero()
    }

    pub fn run(&mut self) -> Failable<Moves> {
        let (cur, next) = match self.side {
            Side::Black => (&mut self.black, &mut self.white),
            Side::White => (&mut self.white, &mut self.black)
        };
        let next_move = match cur {
            Some(ai) => {
                if self.side != *self.board.side() {
                    Moves::default()
                } else {
                    ai.run()?
                }
            },
            None => loop {
                if self.side != *self.board.side() {
                    info!("Pass with [pa]");
                }
                match Game::from_stdin(&self.board)? {
                    Err(err) => error!("{}", err),
                    Ok(next_move) => break next_move
                }
            }
        };
        self.board.place_mut(next_move);
        if let Some(ai) = next.as_mut() {
            ai.update(next_move);
        }
        self.side = !self.side;
        Ok(next_move)
    }
}

