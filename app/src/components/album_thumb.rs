use crate::{
    components::IconShare,
    hooks::{use_photo_src::use_photo_src, use_shares},
    models::Album,
};
use upholi_lib::PhotoVariant;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AlbumThumbProps {
    pub album: Album,
}

#[function_component(AlbumThumb)]
pub fn album_thumb(props: &AlbumThumbProps) -> Html {
    let (shares, _) = use_shares();
    let src = use_photo_src(
        &props.album.thumbnail_photo_id.clone().unwrap_or_default(),
        PhotoVariant::Thumbnail,
    );

    let is_shared = shares.iter().any(|share| share.album_id == props.album.id);

    html! {
        <div class={"album-thumb"}>
            <div
                class={"img"}
                style={format!("background-image: url({})", &(*src))}/>
            <div class={"title"}>
                <span>{&props.album.title}</span>
                if is_shared {
                    <IconShare/>
                }
            </div>
        </div>
    }
}
