use yew::prelude::*;

#[hook]
pub fn use_library_photos() -> (UseStateHandle<Vec<crate::models::LibraryPhoto>>, Callback<()>) {
    let photos = use_state(Vec::new);

    let refresh_photos = {
        let photos = photos.clone();

        Callback::from(move |_| {
            let photos = photos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let library_photos = crate::WASM_CLIENT.get_library_photos().await.unwrap();
                photos.set(library_photos);
            });
        })
    };

    {
        let refresh_photos = refresh_photos.clone();
        use_effect_with_deps(move |_| refresh_photos.emit(()), ());
    }

    (photos, refresh_photos)
}
