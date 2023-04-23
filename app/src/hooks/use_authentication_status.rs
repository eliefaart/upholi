use yew::prelude::*;

#[derive(PartialEq)]
pub enum AuthenticationStatus {
    Fetching,
    Authenticated,
    Unauthenticated,
}

#[hook]
pub fn use_authentication_status() -> UseStateHandle<AuthenticationStatus> {
    let state = use_state(|| AuthenticationStatus::Fetching);

    {
        let state = state.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let authenticated = crate::WASM_CLIENT.is_authenticated().await.unwrap_or_default();

                    state.set(if authenticated {
                        AuthenticationStatus::Authenticated
                    } else {
                        AuthenticationStatus::Unauthenticated
                    });
                });
            },
            (),
        );
    }

    state
}
