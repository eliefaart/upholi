use yew::prelude::*;

#[hook]
pub fn use_library_photos() -> (UseStateHandle<Vec<crate::models::LibraryPhoto>>, Callback<()>) {
    let photos = use_state(|| vec![]);

    let refresh_photos_photos = photos.clone();
    let refresh_photos = Callback::from(move |_| {
        let refresh_photos_photos = refresh_photos_photos.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let library_photos = crate::WASM_CLIENT.get_library_photos().await.unwrap();
            refresh_photos_photos.set(library_photos);
        });
    });

    //let use_effect_photos = photos.clone();
    let use_effect_refresh_photos = refresh_photos.clone();
    use_effect_with_deps(move |_| use_effect_refresh_photos.emit(()), ());

    return (photos, refresh_photos);
}
