use crate::field;
use super::Cell;

static mut PLAYER: Cell = Cell::Empty;

#[inline]
pub unsafe fn reset() {
    PLAYER = Cell::Empty;
}

pub unsafe fn ask() {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    println!("Enter type(X\\O):");
    while PLAYER == Cell::Empty {
        let _ = stdin.read_line(&mut buf);
        match buf.as_str() {
            "X\n" => PLAYER = Cell::X,
            "O\n" => PLAYER = Cell::O,
            _ => println!("Wrong, try again.")
        }
        buf.clear()
    }
}

#[inline]
pub unsafe fn get() -> Cell {
    PLAYER
}

pub unsafe fn turn() -> bool {
    let mut buf = String::new();

    if !field::random(field::FullRandomIter::new()).is_found() {
        println!("~~~ Tie! ~~~");
        std::process::exit(0)
    }

    println!("Enter coords(x y, e.g. 0 1) or command(q)");
    let _ = std::io::stdin().read_line(&mut buf);
    if buf == "q\n" {
        println!("Exiting...");
        std::process::exit(0);
    }
    let mut x = buf.chars().nth(0).unwrap() as u8;
    let mut y = buf.chars().nth(2).unwrap() as u8;
    if (x < ('0' as u8)) || (x > ('2' as u8)) || (y < ('0' as u8)) || (y > ('2' as u8))  { return false }

    x -= '0' as u8;
    y -= '0' as u8;
    if !field::show_changes(x, y, PLAYER) { return false }
    if field::pattern(PLAYER, PLAYER, PLAYER).is_found() {
        println!("~~~ You win! ~~~");
        std::process::exit(0)
    }
    true
}
