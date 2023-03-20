use crate::WASM_CLIENT;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PhotoProps {
    pub class: Option<String>,
    pub photo_id: String,
}

#[function_component(Photo)]
pub fn photo(props: &PhotoProps) -> Html {
    let src = use_state(|| String::new());
    let class = props.class.clone().unwrap_or_default();

    let use_effect_src = src.clone();
    let use_effect_photo_id = props.photo_id.clone();
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let photo_src = WASM_CLIENT
                    .get_photo_image_src(&use_effect_photo_id, upholi_lib::PhotoVariant::Thumbnail)
                    .await
                    .unwrap();
                use_effect_src.set(photo_src);
            });
        },
        (),
    );

    html! {
        <div
            class={format!("photo {class}", )}
            style={format!("background-image: url({})", (*src).clone())}
        />
    }
}
