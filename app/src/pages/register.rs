use crate::{components::Form, Route, WASM_CLIENT};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(RegisterPage)]
pub fn register() -> Html {
    let username_ref = use_node_ref();
    let password_ref = use_node_ref();
    let navigator = use_navigator().unwrap();

    let on_click_create = {
        let username_ref = username_ref.clone();
        let password_ref = password_ref.clone();

        move |_| {
            if let (Some(username_input), Some(password_input)) = (
                username_ref.cast::<HtmlInputElement>(),
                password_ref.cast::<HtmlInputElement>(),
            ) {
                let username = username_input.value();
                let password = password_input.value();
                let navigator = navigator.clone();

                if !password.is_empty() && !username.is_empty() {
                    wasm_bindgen_futures::spawn_local(async move {
                        WASM_CLIENT.register(&username, &password).await.unwrap();
                        navigator.push(&Route::Home)
                    });
                }
            }
        }
    };

    html! {
        <Form title="Create new user" on_submit={on_click_create}>
            <label>{"Username"}
                <input ref={username_ref} type="text"/>
            </label>
            <label>{"Password"}
                <input ref={password_ref} type="password"/>
            </label>
        </Form>
    }
}
