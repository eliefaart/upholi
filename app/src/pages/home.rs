use crate::{
    components::{gallery::Gallery, layouts::PageLayout},
    models::AlbumPhoto,
    WASM_CLIENT,
};
use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let photos = use_state(|| vec![]);
    let selected_photos = use_state(|| Vec::<String>::new());

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

    let photos: Vec<AlbumPhoto> = (*photos).clone().into_iter().map(|photo| photo.into()).collect();
    let on_selection_changed_selected_photos = selected_photos.clone();
    let on_selection_changed = move |ids: Vec<String>| {
        on_selection_changed_selected_photos.set(ids.clone());
    };

    let n_photos_selected = (*selected_photos).len();
    let header_actions = match n_photos_selected {
        0 => html! {},
        _ => html! {
            <>
                <button>{"Add to album"}</button>
                <button>{"Delete"}</button>
                <button>{n_photos_selected} {"selected X"}</button>
            </>
        },
    };

    html! {
        <PageLayout header_actions={header_actions}>
            <Gallery photos={photos} on_selection_changed={on_selection_changed}/>
        </PageLayout>
    }
}
