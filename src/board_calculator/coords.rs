#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Coords {
    pub x: isize,
    pub y: isize,
}

impl Coords {
    pub const fn new(x: isize, y: isize) -> Coords {
        Coords { x, y }
    }

    pub const fn from_tuple(x: &(isize, isize)) -> Coords {
        Coords { x: x.0, y: x.1 }
    }

    pub const fn of(coords: &Coords) -> Coords {
        Coords { x: coords.x, y: coords.y }
    }

    pub fn add(&self, other: &Coords) -> Coords {
        Coords::new(self.x + other.x, self.y + other.y)
    }
}