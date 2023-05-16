use crate::{models::AuthStatus, Route};
use bounce::use_atom;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_router::prelude::use_navigator;

#[derive(Properties, PartialEq)]
pub struct RequireAuthProps {
    pub children: Children,
}

#[function_component(RequireAuth)]
pub fn require_auth(props: &RequireAuthProps) -> Html {
    let state = use_atom::<AuthStatus>();
    let navigator = use_navigator().unwrap();

    {
        let state = state.clone();

        use_effect_once(move || {
            if *state != AuthStatus::Authenticated {
                wasm_bindgen_futures::spawn_local(async move {
                    let authenticated = crate::WASM_CLIENT.is_authenticated().await.unwrap_or_default();

                    state.set(if authenticated {
                        AuthStatus::Authenticated
                    } else {
                        AuthStatus::Unauthenticated
                    });
                });
            }

            || {}
        });
    }

    use_effect_with_deps(
        move |status| {
            if status == &AuthStatus::Unauthenticated {
                navigator.push(&Route::Login);
            }
        },
        (*state).clone(),
    );

    html! {
        if *state == AuthStatus::Authenticated {
            <>{props.children.clone()}</>
        }
    }
}
