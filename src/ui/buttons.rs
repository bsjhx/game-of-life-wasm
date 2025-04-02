use yew::{Callback, Html, Properties, function_component, html};

#[derive(Properties, PartialEq)]
pub struct ButtonsListProps {
    pub(crate) on_next_frame: Callback<()>,
    pub(crate) on_clear: Callback<()>,
    pub(crate) on_play: Callback<()>,
}

#[function_component(Buttons)]
pub fn buttons(ButtonsListProps { on_next_frame, on_clear, on_play }: &ButtonsListProps) -> Html {
    let on_next_frame_click = {
        let on_click = on_next_frame.clone();
        Callback::from(move |_| on_click.emit(()))
    };

    let on_clear_click = {
        let on_click = on_clear.clone();
        Callback::from(move |_| on_click.emit(()))
    };

    let on_play_click = {
        let on_click = on_play.clone();
        Callback::from(move |_| on_click.emit(()))
    };

    html! {
            <div class="grid large-space">
                <div class="s4">
                    <button type="button s1" onclick={on_next_frame_click}>{ "Next frame" }</button>
                </div>

                <div class="s4">
                    <button type="button s1"  onclick={on_clear_click}>{ "Clear" }</button>
                </div>

                <div class="s4">
                    <button type="button s1"  onclick={on_play_click}>{ "Play" }</button>
                </div>
            </div>
        }
}
