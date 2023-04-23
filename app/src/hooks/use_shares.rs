use crate::models::LibraryShare;
use yew::prelude::*;

#[hook]
pub fn use_shares() -> UseStateHandle<Vec<LibraryShare>> {
    let state = use_state(Vec::new);

    {
        let state = state.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let shares = crate::WASM_CLIENT.get_shares().await.unwrap();
                    state.set(shares);
                });
            },
            (),
        );
    }

    state
}
