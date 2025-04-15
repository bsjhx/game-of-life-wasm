use log::info;
use crate::board_calculator::board_calculator::Board;
use crate::board_calculator::coords::Coords;
use web_sys::MouseEvent;
use yew::{Callback, Html, Properties, function_component, html, UseStateHandle};

#[derive(Clone, PartialEq, Debug)]
pub struct Cell {
    pub x: isize,
    pub y: isize,
    pub is_alive: bool,
}

#[derive(Properties, PartialEq)]
pub struct SquareListProps {
    pub board: UseStateHandle<Board>,
    pub on_click: Callback<(isize, isize)>,
}

#[function_component(SquaresList)]
pub fn squares_list(SquareListProps { board, on_click }: &SquareListProps) -> Html {
    (0..30).flat_map(|i| {
        (0..30).map(move |j| {
            let on_square_click = {
                let on_click = on_click.clone();
                Callback::from(move |_| on_click.emit((i, j)))
            };
            if board.is_cell_alive(&Coords::new(i, j)) {
                info!("zywe: {}:{}", i, j);
                alive_square(on_square_click)
            } else {
                dead_square(on_square_click)
            }
        })
    }).collect::<Html>()
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
