use core::fmt;

pub type Pos = u8;

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Cell {
    Empty,
    X,
    O
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use fmt::Write;

        f.write_char(match self {
            Self::Empty => '_',
            Self::X => 'X',
            Self::O => 'O'
        })
    }
}

