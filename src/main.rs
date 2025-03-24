mod board;

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="grid-container">
            <SquaresList squares={squares_generator()} />
        </div>
    }
}

fn squares_generator() -> Vec<Square> {
    let mut result = vec![];
    for i in 0..1950 {
        if i % 50 == 0 {
            result.push(Square { id: i, is_alive: true })
        } else {
            result.push(Square { id: i, is_alive: false })
        }
    }

    result
}

#[derive(Clone, PartialEq)]
struct Square {
    id: usize,
    is_alive: bool
}

#[derive(Properties, PartialEq)]
struct SquareListProps {
    squares: Vec<Square>,
}

#[function_component(SquaresList)]
fn squares_list(SquareListProps { squares }: &SquareListProps) -> Html {
    squares.iter().map(
        |s| {
            if s.is_alive {
                active_square()
            } else {
                square()
            }
        }
    ).collect()
}

fn square() -> Html {
    html! {
        <div class="square"></div>
    }
}

fn active_square() -> Html {
    html! {
        <div class="active-square"></div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
