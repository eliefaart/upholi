use crate::{hooks::use_photo_src::use_photo_src, models::Album};
use upholi_lib::PhotoVariant;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AlbumThumbProps {
    pub album: Album,
}

#[function_component(AlbumThumb)]
pub fn album_thumb(props: &AlbumThumbProps) -> Html {
    let src = use_photo_src(
        &props.album.thumbnail_photo_id.clone().unwrap_or_default(),
        PhotoVariant::Thumbnail,
    );

    html! {
        <div class={"album-thumb"}>
            <div
                class={"img"}
                style={format!("background-image: url({})", &(*src))}/>
            <span class={"title"}>{&props.album.title}</span>
        </div>
    }
}
