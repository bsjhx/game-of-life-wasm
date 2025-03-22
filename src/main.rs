mod board;

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! { <h1>{ "Hello, Rust Frontend! This is YOLO" }</h1> }
}

fn main() {
    yew::Renderer::<App>::new().render();
}