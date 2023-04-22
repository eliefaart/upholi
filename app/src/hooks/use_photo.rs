use crate::models::Photo;
use yew::prelude::*;

#[hook]
pub fn use_photo(photo_id: &str) -> UseStateHandle<Option<Photo>> {
    let state = use_state(|| None);

    {
        let state = state.clone();

        use_effect_with_deps(
            move |photo_id| {
                let photo_id = photo_id.to_owned();

                wasm_bindgen_futures::spawn_local(async move {
                    let photo = crate::WASM_CLIENT
                        .get_photo(&photo_id)
                        .await
                        .expect("Failed to get photo info");
                    state.set(Some(photo));
                });
            },
            photo_id.to_owned(),
        );
    }

    return state;
}
