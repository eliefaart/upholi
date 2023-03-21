use crate::hooks::use_photo_src::use_photo_src;
use upholi_lib::PhotoVariant;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PhotoProps {
    pub class: Option<String>,
    pub photo_id: String,
}

#[function_component(Photo)]
pub fn photo(props: &PhotoProps) -> Html {
    let src = use_photo_src(props.photo_id.clone(), PhotoVariant::Thumbnail);
    let class = props.class.clone().unwrap_or_default();

    html! {
        <div
            class={format!("photo {class}", )}
            style={format!("background-image: url({})", (*src).clone())}
        />
    }
}
