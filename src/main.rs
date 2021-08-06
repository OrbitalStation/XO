#![feature(try_trait_v2)]
#![feature(control_flow_enum)]
#![feature(const_raw_ptr_deref)]

use std::{
    ops::{Try, ControlFlow, FromResidual},
    process::exit
};
use rand::Rng;
use xo::{
    field,
    Pos,
    Cell
};

static mut PLAYER: Cell = Cell::Empty;
static mut AI: Cell = Cell::Empty;

unsafe fn change(x: Pos, y: Pos, c: Cell) -> bool {
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

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq)]
enum FoundResult {
    Found(Pos, Pos),
    No
}

impl FoundResult {
    pub fn is_found(&self) -> bool {
        match self {
            Self::Found(_, _) => true,
            Self::No => false
        }
    }
}

impl FromResidual for FoundResult {
    fn from_residual(residual: (Pos, Pos)) -> Self {
        Self::Found(residual.0, residual.1)
    }
}

impl Try for FoundResult {
    type Output = ();
    type Residual = (Pos, Pos);

    fn from_output(_: Self::Output) -> Self {
        Self::No
    }

    fn branch(self) -> ControlFlow <Self::Residual, Self::Output> {
        match self {
            FoundResult::Found(x, y) => ControlFlow::Break((x, y)),
            FoundResult::No => ControlFlow::Continue(())
        }
    }
}

unsafe fn pattern(pat: (Cell, Cell, Cell)) -> FoundResult {
    let locate = |pos: ((Pos, Pos), (Pos, Pos), (Pos, Pos))| -> FoundResult {
        let h1 = |x: Cell| -> FoundResult {
            if pat.0 == x { FoundResult::Found(pos.0.0, pos.0.1) }
            else if pat.1 == x { FoundResult::Found(pos.1.0, pos.1.1) }
            else if pat.2 == x { FoundResult::Found(pos.2.0, pos.2.1) }
            else { FoundResult::No }
        };

        h1(Cell::Empty)?;
        h1(AI)?;
        h1(PLAYER)
    };

    if field::cmp(0, 0, pat.0) {
        if field::cmp(1, 0, pat.1) && field::cmp(2, 0, pat.2){
            return locate(((0, 0), (1, 0), (2, 0)))
        }
        if field::cmp(0, 1, pat.1) && field::cmp(0, 2, pat.2) {
            return locate(((0, 0), (0, 1), (0, 2)))
        }
        if field::cmp(1, 1, pat.1) && field::cmp(2, 2, pat.2) {
            return locate(((0, 0), (1, 1), (2, 2)))
        }
    }
    if field::cmp(2, 0, pat.0) {
        if field::cmp(2, 1, pat.1) && field::cmp(2, 2, pat.2) {
            return locate(((2, 0), (2, 1), (2, 2)))
        }
        if field::cmp(1, 1, pat.1) && field::cmp(0, 2, pat.2) {
            return locate(((2, 0), (1, 1), (0, 2)))
        }
    }
    if field::cmp(2, 2, pat.0) {
        if field::cmp(1, 2, pat.1) && field::cmp(0, 2, pat.2) {
            return locate(((2, 2), (1, 2), (0, 2)))
        }
    }
    if field::cmp(1, 0, pat.0) && field::cmp(1, 1, pat.1) && field::cmp(1, 2, pat.2) {
        return locate(((1, 0), (1, 1), (1, 2)));
    } else if field::cmp(0, 1, pat.0) && field::cmp(1, 1, pat.1) && field::cmp(2, 1, pat.2) {
        return locate(((0, 1), (1, 1), (2, 1)));
    }
    FoundResult::No
}

unsafe fn rand_cell() -> FoundResult {
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

    if pattern((PLAYER, AI, PLAYER)).is_found() {
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

unsafe fn ai() -> FoundResult {
    use Cell::Empty;

    pattern((AI, Empty, AI))?;
    pattern((AI, AI, Empty))?;
    pattern((Empty, AI, AI))?;

    pattern((PLAYER, Empty, PLAYER))?;
    pattern((PLAYER, PLAYER, Empty))?;
    pattern((Empty, PLAYER, PLAYER))?;

    rand_cell()
}

unsafe fn game() {
    let mut buf = String::new();

    field::create(3, 3);

    println!("Enter type(X\\O):");
    while PLAYER == Cell::Empty {
        let _ = std::io::stdin().read_line(&mut buf);
        match buf.as_str() {
            "X\n" => {
                PLAYER = Cell::X;
                AI = Cell::O;
            },
            "O\n" => {
                PLAYER = Cell::O;
                AI = Cell::X;
            },
            _ => println!("Wrong, try again.")
        }
        buf.clear()
    }

    change(0, 0, Cell::Empty);

    let mut was = PLAYER == Cell::X;
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

            if !change(_0 - ('0' as u8), _2 - ('0' as u8), PLAYER) { continue }
            if pattern((PLAYER, PLAYER, PLAYER)).is_found() {
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
        change(x, y, AI);
        if pattern((AI, AI, AI)).is_found() {
            println!("~~~ AI win! ~~~");
            return
        }
    }
}

fn main() {
    loop {
        unsafe { game() }
        println!("Would you like to play one more game?(y/n)");
        {
            let mut buf = String::new();
            let _ = std::io::stdin().read_line(&mut buf);
            if buf != "\n" && buf != "Y\n" && buf != "y\n" {
                println!("Goodbye!");
                break
            }
            unsafe {
                PLAYER = Cell::Empty;
                AI = Cell::Empty
            }
        }
    }
}
