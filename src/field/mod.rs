mod types;
mod impls;

use crate::{Cell, Pos, FoundResult};

#[inline]
pub fn create(width: Pos, height: Pos) {
    unsafe { impls::create(width, height) }
}

#[inline]
pub fn set(x: Pos, y: Pos, v: Cell) {
    unsafe { impls::set(x, y, v) }
}

#[inline]
pub fn get(x: Pos, y: Pos) -> Cell {
    unsafe { impls::get(x, y) }
}

#[inline]
pub fn width() -> Pos {
    unsafe { impls::width() }
}

#[inline]
pub fn height() -> Pos {
    unsafe { impls::height() }
}

#[inline]
pub fn cmp(x: Pos, y: Pos, v: Cell) -> bool {
    unsafe { impls::cmp(x, y, v) }
}

#[inline]
pub fn pattern(a: Cell, b: Cell, c: Cell) -> FoundResult {
    unsafe { impls::pattern((a, b, c)) }
}
