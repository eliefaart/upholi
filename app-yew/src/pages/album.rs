use crate::{components::gallery::Gallery, hooks::use_album::use_album};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AlbumPageProps {
    pub id: String,
}

#[function_component(AlbumPage)]
pub fn album_page(props: &AlbumPageProps) -> Html {
    let album = use_album(props.id.clone());

    match (*album).clone() {
        Some(album) => {
            let photo_ids = album.photos.into_iter().map(|photo| photo.id).collect::<Vec<String>>();

            html! {
                <>
                    <h1>{ album.title.clone() }</h1>
                    <Gallery photo_ids={photo_ids}/>
                </>
            }
        }
        None => {
            html! {}
        }
    }
}
