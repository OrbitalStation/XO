use crate::{Cell, Pos};

pub struct Field {
    pub cells: Vec <Cell>,
    pub width: Pos
}

impl Field {
    pub const fn stub() -> Self {
        Self {
            cells: Vec::new(),
            width: 0
        }
    }
}
