#![feature(try_trait_v2)]
#![feature(control_flow_enum)]
#![feature(const_raw_ptr_deref)]

use std::ops::{Try, ControlFlow, FromResidual};
use std::process::exit;
use rand::Rng;

static mut FIELD: [[u8; 3]; 3] = [[0; 3]; 3];
static mut PLAYER: u8 = 0;
static mut AI: u8 = 0;

fn utoc(c: u8) -> char {
    match c {
        0 => '_',
        1 => 'X',
        _ => 'O'
    }
}

unsafe fn change(x: usize, y: usize, c: u8) -> bool {
    let one = |_y: usize, no: bool| {
        if y == _y && no {
            FIELD[x][y] = c
        }
        print!("{} {} {}", utoc(FIELD[0][_y]), utoc(FIELD[1][_y]), utoc(FIELD[2][_y]));
    };

    if FIELD[x][y] != 0 {
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
enum AIPR {
    Found(u8, u8),
    No
}

impl AIPR {
    pub fn is_found(self) -> bool {
        match self {
            Self::Found(_, _) => true,
            Self::No => false
        }
    }
}

impl FromResidual for AIPR {
    fn from_residual(residual: (u8, u8)) -> Self {
        Self::Found(residual.0, residual.1)
    }
}

impl Try for AIPR {
    type Output = ();
    type Residual = (u8, u8);

    fn from_output(_: Self::Output) -> Self {
        Self::No
    }

    fn branch(self) -> ControlFlow <Self::Residual, Self::Output> {
        match self {
            AIPR::Found(x, y) => ControlFlow::Break((x, y)),
            AIPR::No => ControlFlow::Continue(())
        }
    }
}

unsafe fn pattern(pat: (u8, u8, u8)) -> AIPR {
    let locate = |pos: ((u8, u8), (u8, u8), (u8, u8))| -> AIPR {
        let h1 = |x: u8| -> AIPR {
            if pat.0 == x { AIPR::Found(pos.0.0, pos.0.1) }
            else if pat.1 == x { AIPR::Found(pos.1.0, pos.1.1) }
            else if pat.2 == x { AIPR::Found(pos.2.0, pos.2.1) }
            else { AIPR::No }
        };

        h1(0)?;
        h1(AI)
    };

    if FIELD[0][0] == pat.0 {
        if FIELD[1][0] == pat.1 && FIELD[2][0] == pat.2 {
            return locate(((0, 0), (1, 0), (2, 0)))
        }
        if FIELD[0][1] == pat.1 && FIELD[0][2] == pat.2 {
            return locate(((0, 0), (0, 1), (0, 2)))
        }
        if FIELD[1][1] == pat.1 && FIELD[2][2] == pat.2 {
            return locate(((0, 0), (1, 1), (2, 2)))
        }
    } else if FIELD[2][0] == pat.0 {
        if FIELD[2][1] == pat.1 && FIELD[2][2] == pat.2 {
            return locate(((2, 0), (2, 1), (2, 2)))
        }
        if FIELD[1][1] == pat.1 && FIELD[0][2] == pat.2 {
            return locate(((2, 0), (1, 1), (0, 2)))
        }
    } else if FIELD[2][2] == pat.0 {
        if FIELD[1][2] == pat.1 && FIELD[0][2] == pat.2 {
            return locate(((2, 2), (1, 2), (0, 2)))
        }
    } else if FIELD[1][0] == pat.0 && FIELD[1][1] == pat.1 && FIELD[1][2] == pat.2 {
        return locate(((1, 0), (1, 1), (1, 2)));
    } else if FIELD[0][1] == pat.0 && FIELD[1][1] == pat.1 && FIELD[2][1] == pat.2 {
        return locate(((0, 1), (1, 1), (2, 1)));
    }
    AIPR::No
}

unsafe fn rand_cell() -> AIPR {
    let mut cells = Vec::new();

    let lambdas = |cells: &mut Vec <(u8, u8)>| {
        let check = |cells: &mut Vec <(u8, u8)>, x: u8, y: u8| {
            if FIELD[x as usize][y as usize] == 0 {
                cells.push((x, y))
            }
        };

        check(cells, 0, 0);
        check(cells, 0, 2);
        check(cells, 2, 0);
        check(cells, 2, 2);
    };

    let ordinary = |cells: &mut Vec <(u8, u8)>| {
        for x in 0u8..3 {
            for y in 0u8..3 {
                if FIELD[x as usize][y as usize] == 0 {
                    cells.push((x, y))
                }
            }
        }
    };

    if FIELD[1][1] == 0 { return AIPR::Found(1, 1) }

    if pattern((PLAYER, AI, PLAYER)).is_found() {
        { ordinary(&mut cells); }
        if cells.is_empty() { lambdas(&mut cells) }
    } else {
        { lambdas(&mut cells); }
        if cells.is_empty() { ordinary(&mut cells) }
    }

    if cells.is_empty() { AIPR::No }
    else {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..cells.len());
        AIPR::Found(cells[x].0, cells[x].1)
    }
}

unsafe fn ai() -> AIPR {

    pattern((AI, 0, AI))?;
    pattern((AI, AI, 0))?;
    pattern((0, AI, AI))?;

    pattern((PLAYER, 0, PLAYER))?;
    pattern((PLAYER, PLAYER, 0))?;
    pattern((0, PLAYER, PLAYER))?;

    rand_cell()
}

fn main() { unsafe {
    let mut buf = String::new();

    println!("Enter type(X\\O):");
    while PLAYER == 0 {
        let _ = std::io::stdin().read_line(&mut buf);
        match buf.as_str() {
            "X\n" => {
                PLAYER = 1;
                AI = 2;
            },
            "O\n" => {
                PLAYER = 2;
                AI = 1;
            },
            _ => println!("Wrong, try again.")
        }
        buf.clear()
    }

    let mut was = PLAYER == 1;
    loop {
        buf.clear();

        if was {
            if !rand_cell().is_found() {
                println!("~~~ Tie! ~~~");
                exit(0);
            }

            println!("Enter cell(x y(each from 0 to 2), e.g. 0 1)");
            let _ = std::io::stdin().read_line(&mut buf);
            if buf.len() != 4 { continue }
            let _0 = buf.chars().nth(0).unwrap() as u8;
            let _2 = buf.chars().nth(2).unwrap() as u8;
            if (_0 < ('0' as u8)) || (_0 > ('2' as u8)) { continue }
            if (_2 < ('0' as u8)) || (_2 > ('2' as u8)) { continue }

            if !change((_0 - ('0' as u8)) as usize, (_2 - ('0' as u8)) as usize, PLAYER) { continue }
            if pattern((PLAYER, PLAYER, PLAYER)).is_found() {
                println!("~~~ You win! ~~~");
                exit(0);
            }
        } else {
            was = true
        }

        println!("AI turn:");
        let (x, y) = match ai() {
            AIPR::Found(x, y) => (x, y),
            AIPR::No => {
                println!("~~~ Tie! ~~~");
                exit(0);
            }
        };
        change(x as usize, y as usize, AI);
        if pattern((AI, AI, AI)).is_found() {
            println!("~~~ AI win! ~~~");
            exit(0);
        }
    }
} }
