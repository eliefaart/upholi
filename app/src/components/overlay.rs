use yew::prelude::*;

use crate::hooks::use_overlay;

#[function_component(Overlay)]
pub fn overlay() -> Html {
    let (visible, _) = use_overlay();

    html! {
        if visible {
            <div class="overlay"></div>
        }
    }
}
