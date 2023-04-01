use crate::hooks::use_photo_src::use_photo_src;
use upholi_lib::PhotoVariant;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PhotoProps {
    pub class: Option<String>,
    pub photo_id: String,
    pub width: Option<f32>,
    pub height: Option<f32>,
    #[prop_or_default]
    pub on_click: Callback<()>,
    #[prop_or_default]
    pub on_context_menu: Callback<MouseEvent>,
}

#[function_component(Photo)]
pub fn photo(props: &PhotoProps) -> Html {
    let src = use_photo_src(&props.photo_id, PhotoVariant::Thumbnail);
    let class = props.class.clone().unwrap_or_default();

    let mut style = format!("background-image: url({}); ", &(*src));
    if let Some(width) = props.width {
        style.push_str(&format!("width: {width}px; "));
    }
    if let Some(height) = props.height {
        style.push_str(&format!("height: {height}px; "));
    }

    let on_click = props.on_click.clone();
    let on_context_menu = props.on_context_menu.clone();
    html! {
        <div
            class={format!("photo {class}")}
            style={style}
            onclick={move |_| on_click.emit(()) }
            oncontextmenu={move |event| on_context_menu.emit(event) }
        />
    }
}

#[derive(Properties, PartialEq)]
pub struct PhotoPreviewProps {
    pub photo_id: String,
}

#[function_component(PhotoPreview)]
pub fn photo_preview(props: &PhotoPreviewProps) -> Html {
    let src = use_photo_src(&props.photo_id, PhotoVariant::Preview);
    let src = (*src).clone();

    html! {
        <img src={src}/>
    }
}
