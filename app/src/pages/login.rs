use crate::{components::Form, hooks::use_authenticated, models::AuthStatus, Route, WASM_CLIENT};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let state = use_authenticated();
    let auth_attempt_made = use_state(|| false);
    let username_ref = use_node_ref();
    let password_ref = use_node_ref();
    let navigator = use_navigator().unwrap();

    let on_click = {
        let state = state.clone();
        let username_ref = username_ref.clone();
        let password_ref = password_ref.clone();
        let auth_attempt_made = auth_attempt_made.clone();

        Callback::from(move |_| {
            if let (Some(username_input), Some(password_input)) = (
                username_ref.cast::<HtmlInputElement>(),
                password_ref.cast::<HtmlInputElement>(),
            ) {
                let username = username_input.value();
                let password = password_input.value();

                if !username.is_empty() && !password.is_empty() {
                    let state = state.clone();
                    let auth_attempt_made = auth_attempt_made.clone();

                    wasm_bindgen_futures::spawn_local(async move {
                        match WASM_CLIENT.login(&username, &password).await {
                            Ok(_) => state.set(AuthStatus::Authenticated),
                            Err(_) => auth_attempt_made.set(true),
                        };
                    });
                }
            }
        })
    };

    {
        use_effect_with_deps(
            move |state| {
                if state == &AuthStatus::Authenticated {
                    navigator.replace(&Route::Home);
                }
            },
            (*state).clone(),
        );
    }

    html! {
        if *state != AuthStatus::Fetching {
            <Form title="Login"
                on_submit={on_click}
                status={if *auth_attempt_made {"Incorrect password"} else {""}}>
                <label>{"Username"}
                    <input ref={username_ref} type="text"/>
                </label>
                <label>{"Password"}
                    <input ref={password_ref} type="password"/>
                </label>
            </Form>
        }
    }
}
