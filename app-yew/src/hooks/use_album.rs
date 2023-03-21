use crate::models::AlbumHydrated;
use yew::prelude::*;

#[hook]
pub fn use_album(album_id: String) -> UseStateHandle<Option<AlbumHydrated>> {
    let album = use_state(|| None);

    let use_effect_album = album.clone();
    let use_effect_album_id = album_id.clone();
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let album = crate::WASM_CLIENT.get_album_full(&use_effect_album_id).await.unwrap();
                use_effect_album.set(Some(album));
            });
        },
        album_id,
    );

    return album;
}
