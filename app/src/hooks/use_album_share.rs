use crate::models::LibraryShare;
use yew::prelude::*;

#[hook]
pub fn use_album_share(album_id: String) -> (UseStateHandle<Option<LibraryShare>>, Callback<()>) {
    let share = use_state(|| None);

    let refresh_share = {
        let share_state = share.clone();
        let album_id = album_id.clone();

        Callback::from(move |_| {
            let share_state = share_state.clone();
            let album_id = album_id.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let share = crate::WASM_CLIENT.get_share_for_album(&album_id).await.unwrap();
                share_state.set(share);
            });
        })
    };

    {
        let refresh_share = refresh_share.clone();
        use_effect_with_deps(move |_| refresh_share.emit(()), album_id);
    }

    (share, refresh_share)
}
