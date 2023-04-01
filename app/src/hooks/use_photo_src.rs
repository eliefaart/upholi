use yew::prelude::*;

#[hook]
pub fn use_photo_src(photo_id: &str, variant: upholi_lib::PhotoVariant) -> UseStateHandle<String> {
    let src = use_state(|| String::new());

    {
        let src = src.clone();
        use_effect_with_deps(
            move |photo_id| {
                let photo_id = photo_id.to_owned();
                wasm_bindgen_futures::spawn_local(async move {
                    let photo_src = crate::WASM_CLIENT.get_photo_image_src(&photo_id, variant).await.unwrap();
                    src.set(photo_src);
                });
            },
            photo_id.to_owned(),
        );
    }

    return src;
}
