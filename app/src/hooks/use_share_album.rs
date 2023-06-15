use crate::models::AlbumHydrated;
use yew::prelude::*;

#[hook]
pub fn use_share_album(share_id: String) -> (UseStateHandle<Option<AlbumHydrated>>, Callback<()>) {
    let state = use_state(|| None);

    let refresh_share = {
        let state = state.clone();
        let share_id = share_id.clone();

        Callback::from(move |_| {
            let state = state.clone();
            let share_id = share_id.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match crate::WASM_CLIENT.get_share_album(&share_id).await {
                    Ok(album) => state.set(Some(album)),
                    Err(_) => state.set(None),
                };
            });
        })
    };

    {
        let refresh_share = refresh_share.clone();
        use_effect_with_deps(move |_| refresh_share.emit(()), share_id);
    }

    (state, refresh_share)
}
