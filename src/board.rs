struct Board {
    fields: Vec<Vec<bool>>
}

fn init_board(size: usize) -> Vec<Vec<bool>> {
    vec![vec![false; size]; size]
}

fn color_pixel(board: &mut Vec<Vec<bool>>, i: usize, j: usize) {
    board[i][j] = true;
}

fn calculate_next_frame(board: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let directions: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut new_board =  init_board(board.len());

    for i in 0..board.len() {
        for j in 0..board[i].len() {
            let mut number_of_alive_cells = 0;

            for direction in directions.iter() {
                let new_ij = get_neighbor(i, j, direction.0, direction.1, board.len(), board.len());

                if let Some((new_i, new_j)) = new_ij {
                    if board[new_i][new_j] {
                        number_of_alive_cells += 1;
                    }
                }
            }

            if number_of_alive_cells == 2 && board[i][j] {
                new_board[i][j] = true;
            }
            if number_of_alive_cells == 3 {
                new_board[i][j] = true;
            }
        }
    }

    new_board
}

fn display_board(board: &Vec<Vec<bool>>) {
    println!();
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            print!("{}", if board[i][j] { "[X]" } else { "[ ]" });
        }
        println!()
    }
}

fn get_neighbor(x: usize, y: usize, dx: isize, dy: isize, width: usize, height: usize) -> Option<(usize, usize)> {
    let new_x = x as isize + dx;
    let new_y = y as isize + dy;

    if new_x >= 0 && new_x < width as isize && new_y >= 0 && new_y < height as isize {
        Some((new_x as usize, new_y as usize))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::board::*;

    #[test]
    fn test_init() {
        let test_size = 10;
        let board = init_board(test_size);
        for i in 0..test_size {
            for j in 0..test_size {
                assert_eq!(board[i][j], false);
            }
        }
    }

    #[test]
    fn test_color_pixel() {
        let test_size = 10;
        let mut board = init_board(test_size);
        color_pixel(&mut board, 0, 0);
        assert_eq!(board[0][0], true);
    }

    #[test]
    fn test_calculate_next_frame() {
        // given
        let test_size = 10;
        let mut board = init_board(test_size);

        // square
        color_pixel(&mut board, 0, 0);
        color_pixel(&mut board, 0, 1);
        color_pixel(&mut board, 1, 0);
        color_pixel(&mut board, 0, 1);

        // when
        board = calculate_next_frame(board);

        // then square should be still there
        assert_eq!(board[0][0], true);
        assert_eq!(board[0][1], true);
        assert_eq!(board[1][0], true);
        assert_eq!(board[0][1], true);

        assert_eq!(board[0][2], false);
        assert_eq!(board[1][2], false);
        assert_eq!(board[2][0], false);
        assert_eq!(board[2][1], false);
    }

    #[test]
    fn test_display_board() {
        let test_size = 10;
        let mut board = init_board(test_size);
        display_board(&board);

        color_pixel(&mut board, 1, 1);
        color_pixel(&mut board, 2, 1);
        color_pixel(&mut board, 3, 1);

        board = calculate_next_frame(board);
        display_board(&board);

        board = calculate_next_frame(board);
        display_board(&board);

        board = calculate_next_frame(board);
        display_board(&board);

        board = calculate_next_frame(board);
        display_board(&board);

        board = calculate_next_frame(board);
        display_board(&board);
    }
}