use crate::{
    components::{
        buttons::{Button, DeleteAlbumButton, IconPosition, RemoveFromAlbumButton, SetAlbumCoverButton},
        drop_upload::{DropUpload, FileUploadProgress},
        gallery::Gallery,
        icons::IconClose,
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
    let selected_photos = use_state(|| Vec::<String>::new());
    let navigator = use_navigator().unwrap();
    let n_photos_selected = (*selected_photos).len();

    let reset_selection = use_memo(
        |selected_photos| {
            let selected_photos = selected_photos.clone();
            Callback::from(move |_: ()| {
                selected_photos.set(vec![]);
            })
        },
        selected_photos.clone(),
    );

    let content = {
        let selected_photos = selected_photos.clone();
        match (*album).clone() {
            Some(album) => {
                html! {
                    <>
                        <h1>{ &album.title }</h1>
                        <Gallery photos={album.photos.clone()} selected_photos={selected_photos}/>
                    </>
                }
            }
            None => {
                html! {}
            }
        }
    };

    let left_album_id = props.id.clone();
    let right_album_id = props.id.clone();
    let on_deleted = move |_| {
        navigator.replace(&Route::Albums);
    };

    let header_actions_left = {
        let selected_photos = selected_photos.clone();
        let on_set_selected_photos = selected_photos.clone();
        let on_removed_selected_photos = selected_photos.clone();
        let refresh_album = refresh_album.clone();

        match n_photos_selected {
            0 => None,
            _ => Some(html! { <>
                {html! {
                    if n_photos_selected == 1 {
                        if let Some(photo_id) = selected_photos.first() {
                            <SetAlbumCoverButton
                                album_id={left_album_id.clone()}
                                photo_id={photo_id.to_string()}
                                on_set={move |_| on_set_selected_photos.set(vec![])}/>
                        }

                    }
                }}
                <RemoveFromAlbumButton
                    album_id={left_album_id}
                    photo_ids={(*selected_photos).clone()}
                    on_removed={move |_| {
                        on_removed_selected_photos.set(vec![]);
                        refresh_album.emit(());
                    }}/>
            </>}),
        }
    };

    let header_actions_right = match n_photos_selected {
        0 => Some(html! {
            <DeleteAlbumButton album_id={right_album_id} on_deleted={on_deleted.clone()}/>
        }),
        _ => Some(html! {
            <>
                <Button label={format!("{n_photos_selected} selected")}
                    on_click={move |_| reset_selection.emit(())}
                    icon_position={IconPosition::Right}>
                    <IconClose/>
                </Button>
            </>
        }),
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
        <PageLayout
            header_actions_left={header_actions_left}
            header_actions_right={header_actions_right}>
            <DropUpload on_upload_status_changed={on_photos_uploaded}>
                {content}
            </DropUpload>

        </PageLayout>
    }
}
