use crate::{
    components::{
        buttons::{
            Button, DeleteAlbumButton, EditAlbumButton, IconPosition, RemoveFromAlbumButton, SetAlbumCoverButton,
        },
        drop_upload::DropUpload,
        gallery::Gallery,
        icons::IconClose,
        layouts::PageLayout,
        BackButton, ShareAlbumButton,
    },
    hooks::{use_album::use_album, use_on_file_upload_finished},
    Route,
};
use use_on_file_upload_finished::FileStatus;
use yew::prelude::*;
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

    {
        let refresh_album = refresh_album.clone();
        use_on_file_upload_finished(Callback::<Vec<FileStatus>>::from(move |_| refresh_album.emit(())));
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

    let header_actions_left = {
        let on_set_selected_photos = selected_photos.clone();
        let on_removed_selected_photos = selected_photos.clone();
        let refresh_album = refresh_album.clone();

        match n_photos_selected {
            0 => Some(html! {<BackButton/>}),
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
                        on_deleted={move |_| { navigator.replace(&Route::Home) }}/>
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

    let content = {
        match (*album).clone() {
            Some(album) => {
                html! {
                    <Gallery photos={album.photos} selected_photos={selected_photos}/>
                }
            }
            None => {
                html! {}
            }
        }
    };

    let album_title = if let Some(album) = (*album).clone() {
        album.title
    } else {
        String::new()
    };

    html! {
        <PageLayout class="album"
            title={album_title}
            header_actions_left={header_actions_left}
            header_actions_right={header_actions_right}>
            <DropUpload target_album_id={props.id.clone()}>
                {content}
            </DropUpload>
        </PageLayout>
    }
}
