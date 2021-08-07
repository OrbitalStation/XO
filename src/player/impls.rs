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

pub unsafe fn turn() {
    let mut buf = String::new();

    if !field::random(field::FullRandomIter::new()).is_found() {
        println!("~~~ Tie! ~~~");
        std::process::exit(0)
    }

    loop {
        buf.clear();
        println!("Enter coords(x y, e.g. 0 1) or command(q)");
        let _ = std::io::stdin().read_line(&mut buf);
        if buf.trim() == "q" {
            println!("Exiting...");
            std::process::exit(0);
        }
        let mut iter = buf.as_str().split_whitespace();
        let x: u8 = match iter.next() {
            Some(x) => match x.parse() {
                Ok(x) => x,
                Err(_) => continue
            },
            None => continue
        };

        let y: u8 = match iter.next() {
            Some(x) => match x.parse() {
                Ok(x) => x,
                Err(_) => continue
            },
            None => continue
        };

        if iter.next().is_some() { continue }
        
        if !field::show_changes(x, y, PLAYER) { continue }
        if field::pattern(PLAYER, PLAYER, PLAYER).is_found() {
            println!("~~~ You win! ~~~");
            std::process::exit(0)
        }

        break
    }
}
