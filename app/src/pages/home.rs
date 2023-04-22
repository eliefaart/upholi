use crate::{
    components::{
        buttons::{AddToAlbumButton, Button, DeletePhotosButton, IconPosition},
        drop_upload::{DropUpload, FileUploadProgress, FileUploadStatus},
        gallery::Gallery,
        icons::IconClose,
        layouts::PageLayout,
    },
    hooks::use_library_photos::use_library_photos,
    models::AlbumPhoto,
};
use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let (photos, refresh_photos) = use_library_photos();
    let selected_photos = use_state(|| Vec::<String>::new());
    let photos: Vec<AlbumPhoto> = (*photos).clone().into_iter().map(|photo| photo.into()).collect();

    let on_click_delete_photos = (*selected_photos).clone();
    let on_click_delete_refresh_photos = refresh_photos.clone();

    let reset_selection = use_memo(
        |selected_photos| {
            let selected_photos = selected_photos.clone();
            Callback::from(move |_: ()| {
                selected_photos.set(vec![]);
            })
        },
        selected_photos.clone(),
    );

    let n_photos_selected = (*selected_photos).len();
    let header_actions_left = match n_photos_selected {
        0 => None,
        _ => Some({
            let on_added_selected_photos = selected_photos.clone();
            let on_deleted_selected_photos = selected_photos.clone();

            html! {
                <>
                    <AddToAlbumButton
                        photo_ids={(*selected_photos).clone()}
                        on_added={move |_| {
                            on_added_selected_photos.set(vec![]);
                        }}
                    />
                    <DeletePhotosButton
                        selected_photos={on_click_delete_photos}
                        on_deleted={move|_| {
                            on_deleted_selected_photos.set(vec![]);
                            on_click_delete_refresh_photos.emit(());
                        }}/>
                </>
            }
        }),
    };
    let header_actions_right = match n_photos_selected {
        0 => None,
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

    let upload_progress_refresh_photos = refresh_photos.clone();

    html! {
        <PageLayout header_actions_left={header_actions_left} header_actions_right={header_actions_right}>
            <DropUpload on_upload_status_changed={move |progress: FileUploadProgress| {
                if progress.status == FileUploadStatus::Done {
                    upload_progress_refresh_photos.emit(());
                }}}>
                <Gallery photos={photos} selected_photos={selected_photos} />
            </DropUpload>
        </PageLayout>
    }
}
