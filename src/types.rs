use core::{
    fmt,
    ops::{Not, Try, ControlFlow, FromResidual}
};

pub type Pos = u8;

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Cell {
    Empty,
    X,
    O
}

impl Not for Cell {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
            Self::Empty => unimplemented!()
        }
    }
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


#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FoundResult {
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

