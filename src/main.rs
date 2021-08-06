use std::process::exit;
use xo::{
    field,
    player,
    ai,
    Pos,
    Cell,
    FoundResult
};

fn game() {
    let mut buf = String::new();

    field::create(3, 3);

    player::ask();
    ai::set();

    field::show_changes(0, 0, Cell::Empty);

    let mut was = player::get() == Cell::X;
    loop {
        buf.clear();

        if was {
            if !field::random(field::FullRandomIter::new()).is_found() {
                println!("~~~ Tie! ~~~");
                return
            }

            println!("Enter cell(x y(each from 0 to 2), e.g. 0 1) or command(quit | exit)");
            let _ = std::io::stdin().read_line(&mut buf);
            if buf.len() != 4 {
                if buf == "quit\n" || buf == "exit\n" {
                    println!("Exiting...");
                    exit(0);
                }
                continue
            }
            let _0 = buf.chars().nth(0).unwrap() as u8;
            let _2 = buf.chars().nth(2).unwrap() as u8;
            if (_0 < ('0' as u8)) || (_0 > ('2' as u8)) { continue }
            if (_2 < ('0' as u8)) || (_2 > ('2' as u8)) { continue }

            if !field::show_changes(_0 - ('0' as u8), _2 - ('0' as u8), player::get()) { continue }
            if field::pattern(player::get(), player::get(), player::get()).is_found() {
                println!("~~~ You win! ~~~");
                return
            }
        } else {
            was = true
        }

        ai::turn()
    }
}

fn main() {
    loop {
        game();

        println!("Would you like to play one more game?(y/n)");
        let mut buf = String::new();
        let _ = std::io::stdin().read_line(&mut buf);
        if buf != "\n" && buf != "Y\n" && buf != "y\n" {
            println!("Goodbye!");
            break
        }

        player::reset();
        ai::reset()
    }
}
