use crate::{hooks::use_photo_src::use_photo_src, models::Album};
use upholi_lib::PhotoVariant;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ShareUrlProps {
    pub share_id: AttrValue,
}

#[function_component(ShareUrl)]
pub fn share_url(props: &ShareUrlProps) -> Html {
    html! {
        <input class="share-url" type="text"
            value={format!("{}/share/{}", crate::ORIGIN.as_str(), props.share_id)}
            readonly={true}/>
    }
}
