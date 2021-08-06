mod impls;

use crate::{Cell, FoundResult};

#[inline]
pub fn reset() {
    unsafe { impls::reset() }
}

#[inline]
pub fn set() {
    unsafe { impls::set() }
}

#[inline]
pub fn get() -> Cell {
    unsafe { impls::get() }
}

#[inline]
pub fn turn() {
    unsafe { impls::turn() }
}
