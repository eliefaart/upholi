use crate::{
    components::{layouts::PageLayout, AlbumThumb, CreateAlbumButton, OpenLibraryButton},
    hooks::use_albums,
};
use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let (albums, refresh_albums) = use_albums();

    let albums = albums
        .iter()
        .map(|album| {
            html! { <AlbumThumb album={album.clone()}/> }
        })
        .collect::<Html>();

    html! {
        <PageLayout class="home"
            //header_actions_left={html!{<h2>{"Home"}</h2>}}
            header_actions_right={html!{<CreateAlbumButton on_created={move |_| refresh_albums.emit(())}/>}}>
            <OpenLibraryButton/>
            <div class="albums">
                {albums}
            </div>
        </PageLayout>
    }
}
