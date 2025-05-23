use yew::Properties;
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

#[derive(Properties, PartialEq, Clone)]
pub struct Board {
    pub cells: Vec<Coords>,
    pub x_size: usize,
    pub y_size: usize,
}

impl Board {
    pub fn new(x_size: usize, y_size: usize) -> Board {
        Board { cells: vec![], x_size, y_size}
    }

    pub fn next_board(&self) -> Board {
        let mut next_board = Board::new(self.x_size, self.y_size);

        for cell in &self.cells {
            for direction in DIRECTIONS {
                let coords = cell.add(&direction);
                let count = self.count_alive_neighbors(&coords);
                if self.is_cell_alive(&coords) && count == 2 {
                    next_board.revive_cell(&coords);
                } else if count == 3 {
                    next_board.revive_cell(&coords)
                }
            }
        }

        next_board
    }

    pub fn is_cell_alive(&self, cell_coords: &Coords) -> bool {
        let index = self
            .cells
            .iter()
            .position(|coords| *coords == *cell_coords);
        index.is_some()
    }

    pub fn revive_cell(&mut self, cell_coords: &Coords) {
        if !self.is_cell_alive(cell_coords) {
            self.cells.push(Coords::of(cell_coords));
        }
    }

    pub fn kill_cell(&mut self, cell_coords: &Coords) {
        let index = self
            .cells
            .iter()
            .position(|coord| *coord == Coords::of(cell_coords));
        if let Some(index) = index {
            self.cells.remove(index);
        }
    }

    fn count_alive_neighbors(&self, cell_coords: &Coords) -> u8 {
        let mut count = 0;

        for direction in DIRECTIONS {
            let neighbor_cell = cell_coords.add(&direction);
            if self.is_cell_alive(&neighbor_cell) {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use crate::board_calculator::board_calculator::{Board};
    use crate::board_calculator::coords::Coords;

    #[test]
    fn test_next_board_rectangle_2_x_2() {
        // given
        let mut board = Board::new(10, 10);

        // when draw rectangle
        board.revive_cell(&Coords::new(0, 0));
        board.revive_cell(&Coords::new(0, 1));
        board.revive_cell(&Coords::new(1, 0));
        board.revive_cell(&Coords::new(1, 1));
        assert_eq!(4, board.cells.len());

        // then next board should still contain same rectangle
        let actual_board = board.next_board();
        assert_eq!(4, actual_board.cells.len());
        assert_eq!(true, actual_board.is_cell_alive(&Coords::new(0, 0)));
        assert_eq!(true, actual_board.is_cell_alive(&Coords::new(0, 1)));
        assert_eq!(true, actual_board.is_cell_alive(&Coords::new(1, 0)));
        assert_eq!(true, actual_board.is_cell_alive(&Coords::new(1, 1)));
    }

    #[test]
    fn test_next_board_line_1_x_3() {
        // given
        let mut board = Board::new(10, 10);

        // when draw rectangle
        board.revive_cell(&Coords::new(0, 0));
        board.revive_cell(&Coords::new(0, 1));
        board.revive_cell(&Coords::new(0, 2));
        assert_eq!(3, board.cells.len());

        // then
        let actual_board = board.next_board();
        assert_eq!(3, actual_board.cells.len());
        assert_eq!(true, actual_board.is_cell_alive(&Coords::new(-1, 1)));
        assert_eq!(true, actual_board.is_cell_alive(&Coords::new(0, 1)));
        assert_eq!(true, actual_board.is_cell_alive(&Coords::new(1, 1)));

        // then
        let actual_board = actual_board.next_board();
        assert_eq!(3, actual_board.cells.len());
        assert_eq!(true, actual_board.is_cell_alive(&Coords::new(0, 0)));
        assert_eq!(true, actual_board.is_cell_alive(&Coords::new(0, 1)));
        assert_eq!(true, actual_board.is_cell_alive(&Coords::new(0, 2)));

        // then
        let actual_board = actual_board.next_board();
        assert_eq!(3, actual_board.cells.len());
        assert_eq!(true, actual_board.is_cell_alive(&Coords::new(-1, 1)));
        assert_eq!(true, actual_board.is_cell_alive(&Coords::new(0, 1)));
        assert_eq!(true, actual_board.is_cell_alive(&Coords::new(1, 1)));
    }

    #[test]
    fn test_revive_and_kill_cell() {
        // given and when
        let mut board = Board::new(10, 10);

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

}
