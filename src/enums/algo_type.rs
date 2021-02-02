use super::*;

#[derive(Clone, Copy, Debug, Derivative, Display, EnumString, EnumVariantNames, PartialEq, Eq)]
#[derivative(Default)]
#[strum(serialize_all = "kebab_case")]
pub enum AlgoType {
	#[derivative(Default)]
	Random,
	Roxanne,
	Mobility
}