mod board;
mod ui;

use yew::prelude::*;
use crate::ui::square::{squares_generator, Square, SquaresList};

#[function_component(App)]
fn app() -> Html {
    let squares = use_state(|| squares_generator());

    let toggle_square = {
        let squares = squares.clone();
        Callback::from(move |clicked_square: Square| {
            let mut new_squares = (*squares).clone();
            if let Some(square) = new_squares.iter_mut().find(|sq| sq.id == clicked_square.id) {
                square.is_alive = !square.is_alive;
            }
            squares.set(new_squares);
        })
    };

    html! {
        <div class="grid-container">
            <SquaresList squares={(*squares).clone()} on_click={toggle_square} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
