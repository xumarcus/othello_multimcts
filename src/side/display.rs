use crate::*;

use std::fmt;

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Side::Black => write!(f, "Black"),
            Side::White => write!(f, "White")
        }
    }   
}
