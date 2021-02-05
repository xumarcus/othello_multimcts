use super::*;

impl error::Error for ParseBoardError {}

#[derive(Clone, Copy, Debug, Derivative, Display, EnumString, PartialEq, Eq)]
#[derivative(Default)]
pub enum ParseBoardError {
    #[derivative(Default)]
	InvalidMove(ParseMovesError),
	InvalidLength(usize),
}
