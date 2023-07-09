use crate::{
    components::{layouts::PageLayout, AlbumThumb, CreateAlbumButton, OpenLibraryButton},
    hooks::use_albums,
};
use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let (albums, refresh_albums) = use_albums();

    let header_actions = {
        let on_created = move |_| refresh_albums.emit(());
        html! {
            <CreateAlbumButton on_created={on_created}/>
        }
    };

    let albums = albums
        .iter()
        .map(|album| {
            html! {
                <AlbumThumb album={album.clone()}/>
            }
        })
        .collect::<Html>();

    html! {
        <PageLayout class="home" header_actions_right={header_actions}>
            <OpenLibraryButton/>
            <hr style={"width: 100%; color: #ad33a5;"}/>
            <div class={"library-items"}>
                {albums}
            </div>
        </PageLayout>
    }
}
