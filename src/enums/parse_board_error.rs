use super::*;

#[derive(Clone, Copy, Debug, Derivative, Display, EnumString, PartialEq, Eq)]
#[derivative(Default)]
pub enum ParseBoardError {
    #[derivative(Default)]
	InvalidFormat(ParseMovesError),
	InvalidMove(Moves),
	InvalidLength(usize),
}

impl error::Error for ParseBoardError {}
