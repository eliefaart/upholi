use use_authentication_status::AuthenticationStatus;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{hooks::use_authentication_status, Route};

#[derive(Properties, PartialEq)]
pub struct RequireAuthProps {
    pub children: Children,
}

#[function_component(RequireAuth)]
pub fn require_auth(props: &RequireAuthProps) -> Html {
    let authentication_status = use_authentication_status();
    let navigator = use_navigator().unwrap();

    use_effect_with_deps(
        move |status| {
            if *status.to_owned() == AuthenticationStatus::Unauthenticated {
                navigator.push(&Route::Login);
            }
        },
        authentication_status.clone(),
    );

    html! {
        if *authentication_status == AuthenticationStatus::Authenticated {
            <>{props.children.clone()}</>
        }
    }
}
