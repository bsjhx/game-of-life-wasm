use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html, Properties};

#[derive(Clone, PartialEq, Debug)]
pub struct Square {
    pub(crate) id: usize,
    pub(crate) is_alive: bool,
}

#[derive(Properties, PartialEq)]
pub struct SquareListProps {
    pub(crate) squares: Vec<Square>,
    pub(crate) on_click: Callback<Square>,
}

#[function_component(SquaresList)]
pub fn squares_list(SquareListProps { squares, on_click }: &SquareListProps) -> Html {
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


pub fn squares_generator() -> Vec<Square> {
    let mut result = vec![];
    for i in 0..1950 {
        result.push(Square {
            id: i,
            is_alive: false,
        });
    }

    result
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
