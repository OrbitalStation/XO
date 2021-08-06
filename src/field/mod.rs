mod types;
mod impls;

use crate::{Cell, Pos, FoundResult};

pub use types::{RandomIter, AngleRandomIter, FullRandomIter};

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

#[inline]
pub fn random(iter: RandomIter) -> FoundResult {
    unsafe { impls::random(iter) }
}

#[inline]
pub fn show_changes(x: Pos, y: Pos, c: Cell) -> bool {
    unsafe { impls::show_changes(x, y, c) }
}
