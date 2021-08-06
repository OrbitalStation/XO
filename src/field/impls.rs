use rand::Rng;
use crate::{ai, player};
use super::{
    types::Field,
    Cell,
    Pos,
    FoundResult,
    RandomIter
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

pub unsafe fn pattern(pat: (Cell, Cell, Cell)) -> FoundResult {
    let locate = |pos: ((Pos, Pos), (Pos, Pos), (Pos, Pos))| -> FoundResult {
        let h1 = |x: Cell| -> FoundResult {
            if pat.0 == x { FoundResult::Found(pos.0.0, pos.0.1) }
            else if pat.1 == x { FoundResult::Found(pos.1.0, pos.1.1) }
            else if pat.2 == x { FoundResult::Found(pos.2.0, pos.2.1) }
            else { FoundResult::No }
        };

        h1(Cell::Empty)?;
        h1(ai::get())?;
        h1(player::get())
    };
    
    if cmp(0, 0, pat.0) {
        if cmp(1, 0, pat.1) && cmp(2, 0, pat.2){
            return locate(((0, 0), (1, 0), (2, 0)))
        }
        if cmp(0, 1, pat.1) && cmp(0, 2, pat.2) {
            return locate(((0, 0), (0, 1), (0, 2)))
        }
        if cmp(1, 1, pat.1) && cmp(2, 2, pat.2) {
            return locate(((0, 0), (1, 1), (2, 2)))
        }
    }
    if cmp(2, 0, pat.0) {
        if cmp(2, 1, pat.1) && cmp(2, 2, pat.2) {
            return locate(((2, 0), (2, 1), (2, 2)))
        }
        if cmp(1, 1, pat.1) && cmp(0, 2, pat.2) {
            return locate(((2, 0), (1, 1), (0, 2)))
        }
    }
    if cmp(2, 2, pat.0) {
        if cmp(1, 2, pat.1) && cmp(0, 2, pat.2) {
            return locate(((2, 2), (1, 2), (0, 2)))
        }
    }
    if cmp(1, 0, pat.0) && cmp(1, 1, pat.1) && cmp(1, 2, pat.2) {
        return locate(((1, 0), (1, 1), (1, 2)));
    } else if cmp(0, 1, pat.0) && cmp(1, 1, pat.1) && cmp(2, 1, pat.2) {
        return locate(((0, 1), (1, 1), (2, 1)));
    }

    FoundResult::No
}

pub unsafe fn random(mut iter: RandomIter) -> FoundResult {
    let mut cells = Vec::new();

    while let Some(x) = iter.rand() {
        cells.push(x)
    }

    if cells.is_empty() { return FoundResult::No }

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..cells.len());
    FoundResult::Found(cells[x].0, cells[x].1)
}

pub unsafe fn show_changes(x: Pos, y: Pos, c: Cell) -> bool {
    let one = |_y: Pos, no: bool| {
        if y == _y && no {
            set(x, y, c)
        }
        print!("{} {} {}", get(0, _y), get(1, _y), get(2, _y));
    };

    if !cmp(x, y, Cell::Empty) {
        println!("Cell is not empty!");
        return false
    }

    println!("=======================");
    for _y in 0..3 {
        print!("\t");
        one(_y, false);
        print!("{}", match _y {
            1 => " => ",
            _ => "    "
        });
        one(_y, true);
        println!()
    }
    println!("\n=======================");
    true
}
