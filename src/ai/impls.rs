use crate::{field, player};
use super::{Cell, FoundResult};

static mut AI: Cell = Cell::Empty;

fn random() -> FoundResult {
    if field::cmp(1, 1, Cell::Empty) { return FoundResult::Found(1, 1) }
    field::random(field::AngleRandomIter::new())?;
    field::random(field::FullRandomIter::new())
}

unsafe fn turn_impl() -> FoundResult {
    use Cell::Empty;

    field::pattern(AI, Empty, AI)?;
    field::pattern(AI, AI, Empty)?;
    field::pattern(Empty, AI, AI)?;

    field::pattern(player::get(), Empty, player::get())?;
    field::pattern(player::get(), player::get(), Empty)?;
    field::pattern(Empty, player::get(), player::get())?;

    random()
}

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

pub unsafe fn turn() {
    println!("AI turn:");

    let (x, y) = match turn_impl() {
        FoundResult::Found(x, y) => (x, y),
        FoundResult::No => {
            println!("~~~ Tie! ~~~");
            std::process::exit(0)
        }
    };

    field::show_changes(x, y, AI);

    if field::pattern(AI, AI, AI).is_found() {
        println!("~~~ AI win! ~~~");
        std::process::exit(0)
    }
}
