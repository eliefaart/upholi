use crate::hooks::use_photo_src::use_photo_src;
use upholi_lib::PhotoVariant;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PhotoProps {
    pub class: Option<String>,
    pub photo_id: String,
    pub width: Option<f32>,
    pub height: Option<f32>,
}

#[function_component(Photo)]
pub fn photo(props: &PhotoProps) -> Html {
    let src = use_photo_src(props.photo_id.clone(), PhotoVariant::Thumbnail);
    let class = props.class.clone().unwrap_or_default();

    let mut style = format!("background-image: url({}); ", &(*src));
    if let Some(width) = props.width {
        style.push_str(&format!("width: {width}px; "));
    }
    if let Some(height) = props.height {
        style.push_str(&format!("height: {height}px; "));
    }

    html! {
        <div
            class={format!("photo {class}", )}
            style={style}
        />
    }
}
