#![feature(try_trait_v2)]
#![feature(control_flow_enum)]
#![feature(const_raw_ptr_deref)]

use std::process::exit;
use rand::Rng;
use xo::{
    field,
    player,
    ai,
    Pos,
    Cell,
    FoundResult
};

fn change(x: Pos, y: Pos, c: Cell) -> bool {
    let one = |_y: Pos, no: bool| {
        if y == _y && no {
            field::set(x, y, c)
        }
        print!("{} {} {}", field::get(0, _y), field::get(1, _y), field::get(2, _y));
    };

    if !field::cmp(x, y, Cell::Empty) {
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

fn rand_cell() -> FoundResult {
    let mut cells = Vec::new();

    let lambdas = |cells: &mut Vec <(Pos, Pos)>| {
        let check = |cells: &mut Vec <(Pos, Pos)>, x: Pos, y: Pos| {
            if field::cmp(x, y, Cell::Empty) {
                cells.push((x, y))
            }
        };

        check(cells, 0, 0);
        check(cells, 0, 2);
        check(cells, 2, 0);
        check(cells, 2, 2);
    };

    let ordinary = |cells: &mut Vec <(Pos, Pos)>| {
        for x in 0..(3 as Pos) {
            for y in 0..(3 as Pos) {
                if field::cmp(x, y, Cell::Empty) {
                    cells.push((x, y))
                }
            }
        }
    };

    if field::cmp(1, 1, Cell::Empty) { return FoundResult::Found(1, 1) }

    if field::pattern(player::get(), ai::get(), player::get()).is_found() {
        { ordinary(&mut cells); }
        if cells.is_empty() { lambdas(&mut cells) }
    } else {
        { lambdas(&mut cells); }
        if cells.is_empty() { ordinary(&mut cells) }
    }

    if cells.is_empty() { FoundResult::No }
    else {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..cells.len());
        FoundResult::Found(cells[x].0, cells[x].1)
    }
}

fn ai() -> FoundResult {
    use Cell::Empty;

    field::pattern(ai::get(), Empty, ai::get())?;
    field::pattern(ai::get(), ai::get(), Empty)?;
    field::pattern(Empty, ai::get(), ai::get())?;

    field::pattern(player::get(), Empty, player::get())?;
    field::pattern(player::get(), player::get(), Empty)?;
    field::pattern(Empty, player::get(), player::get())?;

    rand_cell()
}

fn game() {
    let mut buf = String::new();

    field::create(3, 3);

    player::ask();
    ai::set();

    change(0, 0, Cell::Empty);

    let mut was = player::get() == Cell::X;
    loop {
        buf.clear();

        if was {
            if !rand_cell().is_found() {
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

            if !change(_0 - ('0' as u8), _2 - ('0' as u8), player::get()) { continue }
            if field::pattern(player::get(), player::get(), player::get()).is_found() {
                println!("~~~ You win! ~~~");
                return
            }
        } else {
            was = true
        }

        println!("AI turn:");
        let (x, y) = match ai() {
            FoundResult::Found(x, y) => (x, y),
            FoundResult::No => {
                println!("~~~ Tie! ~~~");
                return
            }
        };
        change(x, y, ai::get());
        if field::pattern(ai::get(), ai::get(), ai::get()).is_found() {
            println!("~~~ AI win! ~~~");
            return
        }
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
