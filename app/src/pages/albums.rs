use crate::{
    components::{album_thumb::AlbumThumb, layouts::PageLayout, CreateAlbumButton},
    hooks::{use_album, use_albums},
    Route, WASM_CLIENT,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(AlbumsPage)]
pub fn albums_page() -> Html {
    let (albums, refresh_albums) = use_albums();
    let navigator = use_navigator().unwrap();

    {
        let albums = albums.clone();
        use_effect_with_deps(
            move |_| {
                let albums = albums.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let library_albums = WASM_CLIENT.get_albums().await.unwrap();
                    albums.set(library_albums);
                });
            },
            (),
        );
    }

    let albums = albums
        .iter()
        .map(|album| {
            let on_click_navigator = navigator.clone();
            let on_click_album_id = album.id.clone();
            let on_click = Callback::from(move |_| {
                on_click_navigator.push(&Route::Album {
                    id: on_click_album_id.clone(),
                })
            });

            html! {
                <div onclick={on_click}>
                    <AlbumThumb album={album.clone()}/>
                </div>
            }
        })
        .collect::<Html>();

    let header_actions = {
        let refresh_albums = refresh_albums.clone();
        let on_created = move |_| refresh_albums.emit(());
        html! {
            <CreateAlbumButton on_created={on_created}/>
        }
    };

    html! {
        <PageLayout header_actions_right={header_actions}>
            <h1>{ "Albums" }</h1>
            <div class={"albums"}>
                {albums}
            </div>
        </PageLayout>
    }
}
