mod board;
mod ui;

use log::info;
use yew::prelude::*;
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
        Callback::from(|s| {
            info!("Next frame");
        })
    };

    let clear = {
        Callback::from(|s| {
            info!("Clear");
        })
    };

    let play = {
        Callback::from(|s| {
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
