use yew::prelude::*;

#[hook]
pub fn use_albums() -> (UseStateHandle<Vec<crate::models::Album>>, Callback<()>) {
    let albums = use_state(|| vec![]);

    let refresh_albums = {
        let albums_state = albums.clone();
        Callback::from(move |_| {
            let albums_state = albums_state.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let albums = crate::WASM_CLIENT.get_albums().await.unwrap();
                albums_state.set(albums);
            });
        })
    };

    {
        let refresh_albums = refresh_albums.clone();
        use_effect_with_deps(move |_| refresh_albums.emit(()), ());
    }

    return (albums, refresh_albums);
}
