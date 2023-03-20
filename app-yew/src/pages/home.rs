use crate::{components::gallery::Gallery, WASM_CLIENT};
use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let photos = use_state(|| vec![]);

    {
        let photos = photos.clone();
        use_effect_with_deps(
            move |_| {
                let photos = photos.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let library_photos = WASM_CLIENT.get_library_photos().await.unwrap();
                    photos.set(library_photos);
                });
            },
            (),
        );
    }

    html! {
        <>
            <Gallery photos={(*photos).clone()}/>
        </>
    }
}
