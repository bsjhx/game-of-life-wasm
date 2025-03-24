mod board;

use yew::prelude::*;

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

fn squares_generator() -> Vec<Square> {
    let mut result = vec![];
    for i in 0..1950 {
        result.push(Square {
            id: i,
            is_alive: false,
        });
    }

    result
}

#[derive(Clone, PartialEq, Debug)]
struct Square {
    id: usize,
    is_alive: bool,
}

#[derive(Properties, PartialEq)]
struct SquareListProps {
    squares: Vec<Square>,
    on_click: Callback<Square>,
}

#[function_component(SquaresList)]
fn squares_list(SquareListProps { squares, on_click }: &SquareListProps) -> Html {
    squares
        .iter()
        .map(|s| {
            let on_video_select = {
                let on_click = on_click.clone();
                let mut s = s.clone();
                Callback::from(move |_| on_click.emit(s.clone()))
            };

            if s.is_alive {
                active_square(on_video_select)
            } else {
                square(on_video_select)
            }
        })
        .collect()
}

fn square(on_video_select: Callback<MouseEvent>) -> Html {
    html! {
        <div class="square" onclick={on_video_select}></div>
    }
}

fn active_square(on_video_select: Callback<MouseEvent>) -> Html {
    html! {
        <div class="active-square" onclick={on_video_select}></div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
