use super::*;

impl error::Error for ParseMovesError {}

#[derive(Clone, Copy, Debug, Derivative, Display, PartialEq, Eq)]
#[derivative(Default)]
pub enum ParseMovesError {
    #[derivative(Default)]
	NotAscii,
	InvalidLength(usize),
	InvalidColumn,
	InvalidRow,
    NotPlaceable(Moves)
}
