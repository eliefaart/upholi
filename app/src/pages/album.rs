use crate::{
    components::{
        buttons::DeleteAlbumButton,
        drop_upload::{DropUpload, FileUploadProgress},
        gallery::Gallery,
        layouts::PageLayout,
    },
    hooks::use_album::use_album,
    Route, WASM_CLIENT,
};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[derive(Properties, PartialEq)]
pub struct AlbumPageProps {
    pub id: String,
}

#[function_component(AlbumPage)]
pub fn album_page(props: &AlbumPageProps) -> Html {
    let (album, refresh_album) = use_album(props.id.clone());
    let navigator = use_navigator().unwrap();

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

    let album_id = props.id.clone();
    let on_deleted = move |_| {
        navigator.replace(&Route::Albums);
    };
    let header_actions = html! {
        <>
            <DeleteAlbumButton album_id={album_id} on_deleted={on_deleted.clone()}/>

        </>
    };

    let on_photos_uploaded_album_id = props.id.clone();
    let on_photos_uploaded_refresh_album = refresh_album.clone();
    let on_photos_uploaded = move |progress: FileUploadProgress| {
        if let Some(photo_id) = progress.uploaded_photo_id {
            let album_id = on_photos_uploaded_album_id.clone();
            let refresh_album = on_photos_uploaded_refresh_album.clone();
            // TODO: Possible race condition modifying album if uploads finish too fast after each other
            wasm_bindgen_futures::spawn_local(async move {
                WASM_CLIENT.add_photos_to_album(&album_id, &vec![photo_id]).await.unwrap();
                refresh_album.emit(());
            });
        }
    };

    html! {
        <PageLayout header_actions_right={header_actions}>
            <DropUpload on_upload_status_changed={on_photos_uploaded}>
                {content}
            </DropUpload>

        </PageLayout>
    }
}
