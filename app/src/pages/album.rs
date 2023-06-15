use std::collections::VecDeque;

use crate::{
    components::{
        buttons::{
            Button, DeleteAlbumButton, EditAlbumButton, IconPosition, RemoveFromAlbumButton, SetAlbumCoverButton,
        },
        drop_upload::{DropUpload, FileUploadProgress},
        gallery::Gallery,
        icons::IconClose,
        layouts::PageLayout,
        FileUploadStatus, ShareAlbumButton,
    },
    hooks::{use_album::use_album, use_on_file_upload_finished},
    Route, WASM_CLIENT,
};
use use_on_file_upload_finished::FileStatus;
use yew::prelude::*;
use yew_hooks::{use_interval, use_queue, UseQueueHandle};
use yew_router::prelude::use_navigator;

#[derive(Properties, PartialEq)]
pub struct AlbumPageProps {
    pub id: String,
}

#[function_component(AlbumPage)]
pub fn album_page(props: &AlbumPageProps) -> Html {
    let (album, refresh_album) = use_album(props.id.clone());
    let selected_photos = use_state(Vec::<String>::new);
    let navigator = use_navigator().unwrap();
    let n_photos_selected = (*selected_photos).len();
    let queue: UseQueueHandle<String> = use_queue(VecDeque::new());

    {
        let queue_empty = queue.current().is_empty();
        let album_id = props.id.clone();
        let refresh_album = refresh_album.clone();
        let queue = queue.clone();

        use_interval(
            move || {
                let mut photo_ids_to_add = vec![];
                while let Some(id) = queue.pop_front() {
                    photo_ids_to_add.push(id);
                }

                let album_id = album_id.clone();
                let refresh_album = refresh_album.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    WASM_CLIENT
                        .add_photos_to_album(&album_id, &photo_ids_to_add)
                        .await
                        .unwrap();
                    refresh_album.emit(());
                });
            },
            if queue_empty { 0 } else { 1000 },
        )
    }

    {
        use_on_file_upload_finished(Callback::from(move |files: Vec<FileStatus>| {
            let photo_ids_to_add: VecDeque<String> = files
                .iter()
                .filter_map(|f| match &f.status {
                    FileUploadStatus::Done { photo_id } | FileUploadStatus::Exists { photo_id } => {
                        Some(photo_id.to_owned())
                    }
                    _ => None,
                })
                .collect();

            for id in photo_ids_to_add {
                queue.push_back(id);
            }
        }));
    }

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
                        <Gallery photos={album.photos} selected_photos={selected_photos}/>
                    </>
                }
            }
            None => {
                html! {}
            }
        }
    };

    let header_actions_left = {
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
                                album_id={props.id.clone()}
                                photo_id={photo_id.to_string()}
                                on_set={move |_| on_set_selected_photos.set(vec![])}/>
                        }

                    }
                }}
                <RemoveFromAlbumButton
                    album_id={props.id.clone()}
                    photo_ids={(*selected_photos).clone()}
                    on_removed={move |_| {
                        on_removed_selected_photos.set(vec![]);
                        refresh_album.emit(());
                    }}/>
            </>}),
        }
    };

    let header_actions_right = {
        let refresh_album_share = refresh_album.clone();
        let refresh_album = refresh_album.clone();

        match n_photos_selected {
            0 => Some(html! {
                <>
                    <ShareAlbumButton
                        album_id={props.id.clone()}
                        on_submitted={move |_| refresh_album_share.emit(()) }/>
                    <EditAlbumButton
                        album_id={props.id.clone()}
                        on_submitted={move |_| refresh_album.emit(()) }/>
                    <DeleteAlbumButton
                        album_id={props.id.clone()}
                        on_deleted={move |_| { navigator.replace(&Route::Albums) }}/>
                </>
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
        }
    };

    let on_photos_uploaded = {
        let album_id = props.id.clone();

        move |progress: FileUploadProgress| {
            if let Some(photo_id) = progress.uploaded_photo_id {
                let album_id = album_id.clone();
                let refresh_album = refresh_album.clone();
                // TODO: Possible race condition modifying album if uploads finish too fast after each other
                wasm_bindgen_futures::spawn_local(async move {
                    WASM_CLIENT.add_photos_to_album(&album_id, &[photo_id]).await.unwrap();
                    refresh_album.emit(());
                });
            }
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
