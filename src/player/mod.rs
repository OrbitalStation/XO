mod impls;

use crate::Cell;

#[inline]
pub fn reset() {
    unsafe { impls::reset() }
}

#[inline]
pub fn ask() {
    unsafe { impls::ask() }
}

#[inline]
pub fn get() -> Cell {
    unsafe { impls::get() }
}
