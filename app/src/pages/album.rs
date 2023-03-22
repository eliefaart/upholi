use crate::{
    components::{gallery::Gallery, layouts::PageLayout},
    hooks::use_album::use_album,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AlbumPageProps {
    pub id: String,
}

#[function_component(AlbumPage)]
pub fn album_page(props: &AlbumPageProps) -> Html {
    let album = use_album(props.id.clone());

    let content = match (*album).clone() {
        Some(album) => {
            html! {
                <>
                    <h1>{ &album.title }</h1>
                    <Gallery photos={album.photos.clone()}/>
                </>
            }
        }
        None => {
            html! {}
        }
    };

    html! {
        <PageLayout>
            {content}
        </PageLayout>
    }
}
