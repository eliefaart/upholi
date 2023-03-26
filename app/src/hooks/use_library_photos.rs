use yew::prelude::*;

#[hook]
pub fn use_library_photos() -> (UseStateHandle<Vec<crate::models::LibraryPhoto>>, Callback<()>) {
    let photos = use_state(|| vec![]);

    let photos_state = photos.clone();
    let refresh_photos = Callback::from(move |_| {
        let photos_state = photos_state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let library_photos = crate::WASM_CLIENT.get_library_photos().await.unwrap();
            photos_state.set(library_photos);
        });
    });

    let use_effect_refresh_photos = refresh_photos.clone();
    use_effect_with_deps(move |_| use_effect_refresh_photos.emit(()), ());

    return (photos, refresh_photos);
}
