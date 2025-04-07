use std::collections::HashMap;
use crate::board_calculator::coords::Coords;

struct Board {
    cells: Vec<Coords>,
}

impl Board {
    fn new() -> Board {
        Board {
            cells: vec![]
        }
    }

    fn revive_cell(&mut self, x: isize, y: isize) {
        if !self.is_cell_alive(x, y) {
            self.cells.push(Coords::new(x, y));
        }
    }

    fn kill_cell(&mut self, x: isize, y: isize) {
        let index = self.cells.iter().position(|coord| *coord == Coords::new(x, y));
        if let Some(index) = index {
            self.cells.remove(index);
        }
    }

    fn is_cell_alive(&self, x: isize, y: isize) -> bool {
        let index = self.cells.iter().position(|coord| *coord == Coords::new(x, y));
        index.is_some()
    }
}

#[cfg(test)]
mod test {
    use crate::board_calculator::board_calculator::Board;
    use crate::board_calculator::coords::Coords;

    #[test]
    fn test_new_board() {
        // given and when
        let board = Board::new();

        // then
        assert_eq!(board.cells.len(), 0);
    }

    #[test]
    fn test_revive_cell() {
        // given and when
        let mut board = Board::new();

        // then
        board.revive_cell(0, 0);
        assert_eq!(board.cells.len(), 1);
        assert_eq!(board.is_cell_alive(0, 0), true);

        board.revive_cell(0, 0);
        assert_eq!(board.cells.len(), 1);
        assert_eq!(board.is_cell_alive(0, 0), true);
    }

}