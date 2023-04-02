use crate::models::AlbumHydrated;
use yew::prelude::*;

#[hook]
pub fn use_album(album_id: String) -> (UseStateHandle<Option<AlbumHydrated>>, Callback<()>) {
    let album = use_state(|| None);

    let album_state = album.clone();
    let refresh_album_album_id = album_id.clone();
    let refresh_album = Callback::from(move |_| {
        let album_state = album_state.clone();
        let refresh_album_album_id = refresh_album_album_id.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let album = crate::WASM_CLIENT.get_album_full(&refresh_album_album_id).await.unwrap();
            album_state.set(Some(album));
        });
    });

    let use_effect_refresh_album = refresh_album.clone();
    use_effect_with_deps(move |_| use_effect_refresh_album.emit(()), album_id);

    return (album, refresh_album);
}
