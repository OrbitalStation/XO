use super::{
    types::Field,
    Cell,
    Pos
};

static mut FIELD: Field = Field::stub();

#[cold]
#[inline(never)]
unsafe fn idx(x: Pos, y: Pos) -> &'static mut Cell {
    &mut FIELD.cells[(y * FIELD.width + x) as usize]
}

pub unsafe fn create(width: Pos, height: Pos) {
    FIELD.cells.clear();
    FIELD.cells.resize((width * height) as usize, Cell::Empty);
    FIELD.width = width;
}

#[inline]
pub unsafe fn set(x: Pos, y: Pos, v: Cell) {
    *idx(x, y) = v;
}

#[inline]
pub unsafe fn get(x: Pos, y: Pos) -> Cell {
    *idx(x, y)
}

#[inline]
pub unsafe fn width() -> Pos {
    FIELD.width
}

pub unsafe fn height() -> Pos {
    FIELD.cells.len() as Pos / FIELD.width
}

#[inline]
pub unsafe fn cmp(x: Pos, y: Pos, v: Cell) -> bool {
    get(x, y) == v
}
