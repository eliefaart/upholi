use crate::hooks::{use_albums, use_photo_src};
use upholi_lib::PhotoVariant;
use yew::prelude::*;

use super::AlbumThumbProps;

#[derive(Properties, PartialEq)]
pub struct PickAlbumProps {
    pub selected_album: UseStateHandle<Option<String>>,
}

#[function_component(PickAlbum)]
pub fn pick_album(props: &PickAlbumProps) -> Html {
    let (albums, _) = use_albums();

    let albums_html = (*albums)
        .iter()
        .map(|album| {
            let on_click = {
                let selected_album = props.selected_album.clone();
                let album_id = album.id.clone();
                Callback::from(move |_| selected_album.set(Some(album_id.clone())))
            };

            let is_selected = match (*props.selected_album).clone() {
                Some(selected_album_id) => selected_album_id == album.id,
                None => false,
            };

            html! {
                <div
                    class={classes!("pick-album-entry", { if is_selected {"selected"} else { "" } } )}
                    onclick={on_click}>
                    <PickAlbumThumb album={album.clone()}/>
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <div class="pick-album">
            {albums_html}
        </div>
    }
}

#[function_component(PickAlbumThumb)]
fn pick_album_thumb(props: &AlbumThumbProps) -> Html {
    let src = use_photo_src(
        &props.album.thumbnail_photo_id.clone().unwrap_or_default(),
        PhotoVariant::Thumbnail,
    );
    html! {
        <div class="pick-album-thumb">
            <div
                class={"pick-album-thumb img"}
                style={format!("background-image: url({})", &(*src))}/>
            <span class="title">{&props.album.title}</span>
        </div>
    }
}
