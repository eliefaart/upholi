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
            html! { <AlbumThumb key={album.id.clone()} album={album.clone()}/> }
        })
        .collect::<Html>();

    html! {
        <PageLayout class="home"
            header_actions_right={html!{<CreateAlbumButton on_created={move |_| refresh_albums.emit(())}/>}}>
            <OpenLibraryButton/>
            <hr style={"width: 100%; border-color: var(--colorText);"}/>
            <div class="albums">
                {albums}
            </div>
        </PageLayout>
    }
}
