mod board;

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="grid-container">
            {squares()}
        </div>
    }
}

fn squares() -> Vec<Html> {
    let mut result = vec![];
    for _i in 0..1950 {
        if _i > 150 && _i < 1200 {
            result.push(active_square())
        } else {
            result.push(square())
        }
    }

    result
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
