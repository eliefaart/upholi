use yew::prelude::*;

#[hook]
pub fn use_is_authorized_for_share(share_id: &str) -> UseStateHandle<Option<bool>> {
    let state = use_state(|| None);

    {
        let share_id = share_id.to_owned();
        let state = state.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let authorized = crate::WASM_CLIENT.is_authorized_for_share(&share_id).await.unwrap();
                    state.set(Some(authorized));
                });
            },
            (),
        );
    }

    state
}
