#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Coords {
    pub x: isize,
    pub y: isize,
}

impl Coords {
    pub const fn new(x: isize, y: isize) -> Coords {
        Coords { x, y }
    }

    pub fn add(&self, other: &Coords) -> Coords {
        Coords::new(self.x + other.x, self.y + other.y)
    }
}