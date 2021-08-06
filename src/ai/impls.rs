use super::{Cell, FoundResult};

static mut AI: Cell = Cell::Empty;

#[inline]
pub unsafe fn reset() {
    AI = Cell::Empty
}

#[inline]
pub unsafe fn set() {
    AI = !crate::player::get();
}

#[inline]
pub unsafe fn get() -> Cell {
    AI
}


