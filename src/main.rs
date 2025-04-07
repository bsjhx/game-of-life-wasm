mod board;
mod ui;
mod board_calculator;

use log::info;
use yew::prelude::*;
use crate::board::{calculate_next_frame, display_board};
use crate::ui::buttons::Buttons;
use crate::ui::square::{squares_generator, Square, SquaresList};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let squares = use_state(|| squares_generator());

    let toggle_square = {
        let squares = squares.clone();
        Callback::from(move |id: usize| {
            let mut new_squares = (*squares).clone();
            if let Some(square) = new_squares.iter_mut().find(|sq| sq.id == id) {
                square.is_alive = !square.is_alive;
            }
            squares.set(new_squares);
        })
    };

    let next_frame = {
        let squares = squares.clone();
        Callback::from(move |_| {
            let values: Vec<bool> = squares.iter().map(|square| {
                square.is_alive
            }).collect();

            let mut board: Vec<Vec<bool>> =vec![vec![false; 5]; 5];
            let mut counter = 0;
            for i in 0..5 {
                for j in 0..5 {
                    board[i][j] = *values.get(counter).unwrap();
                    counter += 1;
                }
            }

            display_board(&board);
            info!("");
            let new_board = calculate_next_frame(board);
            info!("{} {}", new_board[0][0], new_board[0][1]);
            info!("{} {}", new_board[1][0], new_board[1][1]);

            counter = 0;
            let mut new_squares = vec![];
            for i in 0..5 {
                for j in 0..5 {
                    new_squares.push(Square {
                        id: counter,
                        is_alive:  new_board[i][j],
                    });
                    counter += 1;
                }
            }
            squares.set(new_squares);
        })
    };

    let clear = {
        Callback::from(|_| {
            info!("Clear");
        })
    };

    let play = {
        Callback::from(|_| {
            info!("Play");
        })
    };

    html! {
        <div>
        <div class="grid-container">
            <SquaresList squares={(*squares).clone()} on_click={toggle_square} />
        </div>
        <Buttons on_next_frame={next_frame} on_clear={clear} on_play={play} />
        </div>
    }
}
