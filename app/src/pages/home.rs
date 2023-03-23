use crate::{
    components::{
        button::{Button, IconPosition},
        gallery::Gallery,
        icons::{IconAddToAlbum, IconClose, IconDelete},
        layouts::PageLayout,
    },
    hooks::use_library_photos::use_library_photos,
    models::AlbumPhoto,
    WASM_CLIENT,
};
use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let (photos, refresh_photos) = use_library_photos();
    let selected_photos = use_state(|| Vec::<String>::new());

    let photos: Vec<AlbumPhoto> = (*photos).clone().into_iter().map(|photo| photo.into()).collect();
    let on_selection_changed_selected_photos = selected_photos.clone();
    let on_selection_changed = move |ids: Vec<String>| {
        on_selection_changed_selected_photos.set(ids.clone());
    };

    let on_click_delete_photos = (*selected_photos).clone();
    let on_click_delete_refresh_photos = refresh_photos.clone();
    let on_click_delete = move |_| {
        let on_click_delete_photos = on_click_delete_photos.clone();
        let on_click_delete_refresh_photos = on_click_delete_refresh_photos.clone();
        wasm_bindgen_futures::spawn_local(async move {
            WASM_CLIENT.delete_photos(&on_click_delete_photos).await.unwrap();
            on_click_delete_refresh_photos.emit(());
        });
    };
    let n_photos_selected = (*selected_photos).len();
    let header_actions_left = match n_photos_selected {
        0 => None,
        _ => Some(html! {
            <>
                <Button label={"Add to album"} on_click={|_|{}}>
                    <IconAddToAlbum/>
                </Button>
                <Button label={"Delete"} on_click={on_click_delete.clone()}>
                    <IconDelete/>
                </Button>
            </>
        }),
    };
    let header_actions_right = match n_photos_selected {
        0 => None,
        _ => Some(html! {
            <>
                <Button label={format!("{n_photos_selected} selected")} on_click={|_|{}} icon_position={IconPosition::Right}>
                    <IconClose/>
                </Button>
            </>
        }),
    };

    html! {
        <PageLayout header_actions_left={header_actions_left} header_actions_right={header_actions_right}>
            <Gallery photos={photos} on_selection_changed={on_selection_changed}/>
        </PageLayout>
    }
}
