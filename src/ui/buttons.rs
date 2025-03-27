use yew::{Callback, Html, Properties, function_component, html};

#[derive(Properties, PartialEq)]
pub struct ButtonsListProps {
    // pub(crate) on_click: Callback<Square>,
}

#[function_component(Buttons)]
pub fn buttons(ButtonsListProps {}: &ButtonsListProps) -> Html {
    html! {
            <div class="grid large-space">
                <div class="s4">
                    <button type="button s1">{ "Next frame" }</button>
                </div>

                <div class="s4">
                    <button type="button s1">{ "Clear" }</button>
                </div>

                <div class="s4">
                    <button type="button s1">{ "Play" }</button>
                </div>
            </div>
        }
}
