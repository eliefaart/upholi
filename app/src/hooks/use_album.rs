use crate::models::AlbumHydrated;
use yew::prelude::*;

#[hook]
pub fn use_album(album_id: String) -> (UseStateHandle<Option<AlbumHydrated>>, Callback<()>) {
    let album = use_state(|| None);

    let refresh_album = {
        let album_state = album.clone();
        let album_id = album_id.clone();

        Callback::from(move |_| {
            let album_state = album_state.clone();
            let album_id = album_id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let album = crate::WASM_CLIENT.get_album_full(&album_id).await.unwrap();
                album_state.set(Some(album));
            });
        })
    };

    let use_effect_refresh_album = refresh_album.clone();
    use_effect_with_deps(move |_| use_effect_refresh_album.emit(()), album_id);

    (album, refresh_album)
}
