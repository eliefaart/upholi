use crate::{
    components::{album_thumb::AlbumThumb, button::Button, icons::IconCreate, layouts::PageLayout},
    Route, WASM_CLIENT,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(AlbumsPage)]
pub fn albums_page() -> Html {
    let albums = use_state(|| vec![]);
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

    let on_click_create_album = |_| {
        wasm_bindgen_futures::spawn_local(async {
            WASM_CLIENT.create_album("Test", vec![]).await.unwrap();
            // TODO: Make use_albums hook and let it return some callback to refresh
        });
    };
    let header_actions = html! {
        <>
            <Button label={"Create album"} on_click={on_click_create_album.clone()}>
                <IconCreate/>
            </Button>
        </>
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
