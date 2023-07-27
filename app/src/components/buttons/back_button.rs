use crate::components::{Button, IconBack};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(BackButton)]
pub fn back_button() -> Html {
    let navigator = use_navigator().unwrap();

    let on_click = use_callback(
        |_, navigator| {
            navigator.back();
        },
        navigator,
    );

    html! {
        <Button label="" on_click={on_click}>
            <IconBack/>
        </Button>
    }
}
