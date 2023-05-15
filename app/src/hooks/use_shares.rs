use crate::models::LibraryShare;
use yew::prelude::*;
use yew_hooks::use_effect_once;

#[hook]
pub fn use_shares() -> (UseStateHandle<Vec<LibraryShare>>, Callback<()>) {
    let state = use_state(Vec::new);

    let refresh = {
        let state = state.clone();

        Callback::from(move |_| {
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let shares = crate::WASM_CLIENT.get_shares().await.unwrap();
                state.set(shares);
            });
        })
    };

    {
        let refresh = refresh.clone();
        use_effect_once(move || {
            refresh.emit(());
            || {}
        });
    }

    (state, refresh)
}
