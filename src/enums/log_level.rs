use super::*;

#[derive(Clone, Copy, Debug, Derivative, EnumString, EnumVariantNames, PartialEq, Eq, PartialOrd, Ord)]
#[derivative(Default)]
#[strum(serialize_all = "kebab_case")]
pub enum LogLevel {
    Silent,
    Minimal,
    Interact,
    #[derivative(Default)]
    Info,
    Debug
}