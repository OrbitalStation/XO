use xo::{field, player, ai, Cell};

fn game() {
    field::create(3, 3);

    player::ask();
    ai::set();

    field::show_changes(0, 0, Cell::Empty);

    if ai::get().is_x() {
        ai::turn()
    }

    loop {
        player::turn();
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
