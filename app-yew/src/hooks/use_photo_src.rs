use yew::prelude::*;

#[hook]
pub fn use_photo_src(photo_id: String, variant: upholi_lib::PhotoVariant) -> UseStateHandle<String> {
    let src = use_state(|| String::new());

    let use_effect_src = src.clone();
    let use_effect_photo_id = photo_id.clone();
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let photo_src = crate::WASM_CLIENT.get_photo_image_src(&use_effect_photo_id, variant).await.unwrap();
                use_effect_src.set(photo_src);
            });
        },
        photo_id,
    );

    return src;
}
