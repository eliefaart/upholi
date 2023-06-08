use crate::{hooks::use_authenticated, models::AuthStatus, Route};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[derive(Properties, PartialEq)]
pub struct RequireAuthProps {
    pub children: Children,
}

#[function_component(RequireAuth)]
pub fn require_auth(props: &RequireAuthProps) -> Html {
    let state = use_authenticated();
    let navigator = use_navigator().unwrap();

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
