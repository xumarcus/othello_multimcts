use super::*;

#[derive(Clone, Copy, Debug, Derivative, Display, EnumString, PartialEq, Eq)]
#[derivative(Default)]
pub enum ParseMovesError {
    #[derivative(Default)]
	NotAscii,
	InvalidLength(usize),
	InvalidColumn,
	InvalidRow
}

impl error::Error for ParseMovesError {}
