mod board_calculator;
mod ui;

use crate::board_calculator::board_calculator::Board;
use crate::board_calculator::coords::Coords;
use crate::ui::buttons::Buttons;
use crate::ui::square::SquaresList;
use gloo::events::EventListener;
use log::info;
use web_sys::window;
use yew::prelude::*;

const X_SIZE: usize = 25;
const Y_SIZE: usize = 30;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let size = use_state(|| {
        (
            window().unwrap().inner_width().unwrap().as_f64().unwrap() as u32,
            window().unwrap().inner_height().unwrap().as_f64().unwrap() as u32,
        )
    });
    {
        let size = size.clone();
        use_effect(move || {
            let listener = EventListener::new(&window().unwrap(), "resize", move |_| {
                let width = window().unwrap().inner_width().unwrap().as_f64().unwrap() as u32;
                let height = window().unwrap().inner_height().unwrap().as_f64().unwrap() as u32;
                size.set((width, height));
            });
            || drop(listener)
        });
    }

    let board = Board::new(10, 10);
    let board = use_state(|| board);

    let toggle_square = toggle_cell(board.clone());
    let next_frame = next_frame(board.clone());
    let clear = clear_board(board.clone());
    let play = play(size.clone());

    html! {
        <div style={format!("--x-size: {}; --y-size: {};", X_SIZE, Y_SIZE)}>
            <div class="grid-container">
                <SquaresList board={board.clone()} on_click={toggle_square} />
            </div>
            <Buttons on_next_frame={next_frame} on_clear={clear} on_play={play} />
        </div>
    }
}

fn toggle_cell(board: UseStateHandle<Board>) -> Callback<(isize, isize)> {
    let board = board.clone();
    Callback::from(move |coords: (isize, isize)| {
        let mut new_board = (*board).clone();
        let coords = &Coords::from_tuple(&coords);
        if new_board.is_cell_alive(coords) {
            new_board.kill_cell(coords);
        } else {
            new_board.revive_cell(coords);
        }
        board.set(new_board);
    })
}

fn next_frame(board: UseStateHandle<Board>) -> Callback<()> {
    let board = board.clone();
    Callback::from(move |_| {
        let new_board = (*board).next_board();
        board.set(new_board);
    })
}

fn clear_board(board: UseStateHandle<Board>) -> Callback<()> {
    let board = board.clone();
    Callback::from(move |_| {
        board.set(Board::new(X_SIZE, Y_SIZE));
    })
}

fn play(size: UseStateHandle<(u32, u32)>) -> Callback<()> {
    let size = size.clone();
    Callback::from(move |_| {
        info!(
            "Play. Dimm: {:?}, Size: {:?}",
            size,
            calculate_board_size(*size)
        );
    })
}

fn calculate_board_size(dimensions: (u32, u32)) -> (u32, u32) {
    ((dimensions.0 / 16), (dimensions.1 / 16))
}
