use crate::models::LibraryShare;
use yew::prelude::*;

#[hook]
pub fn use_shares() -> UseStateHandle<Vec<LibraryShare>> {
    let shares = use_state(|| vec![]);

    let use_effect_shares = shares.clone();
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let shares = crate::WASM_CLIENT.get_shares().await.unwrap();
                use_effect_shares.set(shares);
            });
        },
        (),
    );

    return shares;
}
