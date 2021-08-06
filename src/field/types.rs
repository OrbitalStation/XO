use crate::{Cell, Pos};

pub struct Field {
    pub cells: Vec <Cell>,
    pub width: Pos
}

impl Field {
    pub const fn stub() -> Self {
        Self {
            cells: Vec::new(),
            width: 0
        }
    }
}

pub struct RandomIter {
    x: Pos,
    y: Pos,
    angle: bool
}

pub struct FullRandomIter;

impl FullRandomIter {
    pub fn new() -> RandomIter {
        RandomIter {
            x: 0,
            y: 0,
            angle: false
        }
    }
}

impl RandomIter {
    pub fn rand(&mut self) -> Option <(Pos, Pos)> {
        if self.angle {
            loop {
                self.x += 1;
                return match self.x {
                    1 if super::cmp(0, 0, Cell::Empty) => Some((0, 0)),
                    2 if super::cmp(0, 2, Cell::Empty) => Some((0, 2)),
                    3 if super::cmp(2, 0, Cell::Empty) => Some((2, 0)),
                    4 if super::cmp(2, 2, Cell::Empty) => Some((2, 2)),
                    _ => None
                }
            }
        } else {
            loop {
                if self.x == super::width() {
                    self.x = 0;
                    self.y += 1
                }
                if self.y == super::height() {
                    return None
                }
                self.x += 1;
                if super::get(self.x - 1, self.y) == Cell::Empty {
                    return Some((self.x - 1, self.y))
                }
            }
        }
    }
}

pub struct AngleRandomIter;

impl AngleRandomIter {
    pub fn new() -> RandomIter {
        RandomIter {
            x: 0,
            y: 0,
            angle: true
        }
    }
}
