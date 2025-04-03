use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html, Properties};

#[derive(Clone, PartialEq, Debug)]
pub struct Square {
    pub id: usize,
    pub is_alive: bool,
}

#[derive(Properties, PartialEq)]
pub struct SquareListProps {
    pub(crate) squares: Vec<Square>,
    pub(crate) on_click: Callback<usize>,
}

#[function_component(SquaresList)]
pub fn squares_list(SquareListProps { squares, on_click }: &SquareListProps) -> Html {
    squares
        .iter()
        .map(|s| {
            let on_square_click = {
                let on_click = on_click.clone();
                let id = s.id;
                Callback::from(move |_| on_click.emit(id))
            };

            if s.is_alive {
                alive_square(on_square_click)
            } else {
                dead_square(on_square_click)
            }
        })
        .collect()
}

pub fn squares_generator() -> Vec<Square> {
    let mut result = vec![];
    for i in 0..25 {
        result.push(Square {
            id: i,
            is_alive: false,
        });
    }

    result
}

fn dead_square(on_square_click: Callback<MouseEvent>) -> Html {
    html! {
        <div class="square" onclick={on_square_click}></div>
    }
}

fn alive_square(on_square_click: Callback<MouseEvent>) -> Html {
    html! {
        <div class="alive-square" onclick={on_square_click}></div>
    }
}
