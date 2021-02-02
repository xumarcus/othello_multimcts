use crate::*;

impl Default for Board {
    fn default() -> Self {
        Self::new(
            0x0000_0008_1000_0000,
            0x0000_0010_0800_0000,
            Side::default()
        ).unwrap()
    }
}