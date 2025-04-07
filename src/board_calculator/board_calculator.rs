use crate::board_calculator::coords::Coords;

const DIRECTIONS: [Coords;8] = [
    Coords::new(-1, -1),
    Coords::new(-1, 0),
    Coords::new(-1, 1),
    Coords::new(0, -1),
    Coords::new(0, 1),
    Coords::new(1, -1),
    Coords::new(1, 0),
    Coords::new(1, 1),
];

struct Board {
    cells: Vec<Coords>,
}

impl Board {
    fn new() -> Board {
        Board { cells: vec![] }
    }

    fn next_board(&self) -> Board {
        let mut next_board = Board::new();

        for cell in &self.cells {

        }

        todo!()
    }

    // fn will_cell_survived(&self, x: usize, y: usize) -> bool {
    //     let mut counter = 0;
    //     for DIRECTION in DIRECTIONS {
    //         if self.is_cell_alive()
    //     }
    //
    //     false
    //
    // }

    fn revive_cell(&mut self, cell: &Coords) {
        if !self.is_cell_alive(cell) {
            self.cells.push(Coords::new(cell.x, cell.y));
        }
    }

    fn kill_cell(&mut self, cell: &Coords) {
        let index = self
            .cells
            .iter()
            .position(|coord| *coord == Coords::new(cell.x, cell.y));
        if let Some(index) = index {
            self.cells.remove(index);
        }
    }

    fn is_cell_alive(&self, cell_coords: &Coords) -> bool {
        let index = self
            .cells
            .iter()
            .position(|coords| *coords == *cell_coords);
        index.is_some()
    }

    fn count_alive_neighbors(&self, cell_coords: &Coords) -> u8 {
        let mut count = 0;

        for DIRECTION in DIRECTIONS {
            let neighbor_cell = cell_coords.add(&DIRECTION);
            if self.is_cell_alive(&neighbor_cell) {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use crate::board_calculator::board_calculator::{Board, DIRECTIONS};
    use crate::board_calculator::coords::Coords;

    #[test]
    fn test_new_board() {
        // given and when
        let board = Board::new();

        // then
        assert_eq!(board.cells.len(), 0);
    }

    #[test]
    fn test_revive_and_kill_cell() {
        // given and when
        let mut board = Board::new();

        assert_eq!(board.cells.len(), 0);
        let cell = Coords::new(0, 0);
        assert_eq!(board.is_cell_alive(&cell), false);

        // then
        board.revive_cell(&cell);
        assert_eq!(board.cells.len(), 1);
        assert_eq!(board.is_cell_alive(&cell), true);

        board.revive_cell(&cell);
        assert_eq!(board.cells.len(), 1);
        assert_eq!(board.is_cell_alive(&cell), true);

        board.kill_cell(&cell);
        assert_eq!(board.cells.len(), 0);
        assert_eq!(board.is_cell_alive(&cell), false);
    }

    #[test]
    fn test_neighbor_count() {
        // given
        let mut board = Board::new();
        let cell00 = Coords::new(0, 0);
        let cell01 = Coords::new(0, 1);
        let cell10 = Coords::new(-1, 0);
        let cell11 = Coords::new(-1, 1);

        // when & then
        let actual = board.count_alive_neighbors(&cell00);
        assert_eq!(0, actual);

        // when & then
        board.revive_cell(&cell01);
        let actual = board.count_alive_neighbors(&cell00);
        assert_eq!(1, actual);

        // when revive rectangle 2x2
        board.revive_cell(&cell00);
        board.revive_cell(&cell01);
        board.revive_cell(&cell10);
        board.revive_cell(&cell11);

        // then
        let actual = board.count_alive_neighbors(&cell00);
        assert_eq!(3, actual);

        // when revive all cells around 0,0
        board.kill_cell(&cell00);
        for direction in DIRECTIONS {
            board.revive_cell(&cell00.add(&direction));
        }

        // then
        let actual = board.count_alive_neighbors(&cell00);
        assert_eq!(8, actual);

        // when kill some of the neighbors
        board.kill_cell(&cell01);
        board.kill_cell(&cell10);
        board.kill_cell(&cell11);

        // then
        let actual = board.count_alive_neighbors(&cell00);
        assert_eq!(5, actual);
    }
}
