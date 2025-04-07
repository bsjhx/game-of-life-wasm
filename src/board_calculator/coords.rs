#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Coords {
    pub x: isize,
    pub y: isize,
}

impl Coords {
    pub fn new(x: isize, y: isize) -> Coords {
        Coords { x, y }
    }
}