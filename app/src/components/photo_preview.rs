use crate::hooks::use_photo_src::use_photo_src;
use upholi_lib::PhotoVariant;
use yew::prelude::*;

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
