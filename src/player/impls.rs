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
