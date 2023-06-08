use crate::models::AuthStatus;
use bounce::use_atom;
use yew::prelude::*;
use yew_hooks::use_effect_once;

#[hook]
pub fn use_authenticated() -> bounce::UseAtomHandle<AuthStatus> {
    let state = use_atom::<AuthStatus>();

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

    state
}
