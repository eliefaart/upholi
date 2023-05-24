use crate::{
    components::{
        Button, DownloadPhotoButton, IconChevronLeft, IconChevronRight, IconClose, PhotoExifButton, PhotoPreview,
    },
    models::AlbumPhoto,
};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct GalleryDetailProps {
    pub photos: Vec<AlbumPhoto>,
    pub photo_id: UseStateHandle<Option<String>>,
}

#[function_component(GalleryDetail)]
pub fn gallery_detail(props: &GalleryDetailProps) -> Html {
    if let Some(photo_id) = (*props.photo_id).clone() {
        let n_photos = props.photos.len();
        let current_idx = props.photos.iter().position(|p| p.id == photo_id).unwrap_or_default();

        let close_button = {
            let photo_id = props.photo_id.clone();
            html! {
                <Button label="" on_click={move |_| photo_id.set(None) }>
                    <IconClose/>
                </Button>
            }
        };

        let prev_button = {
            let target_photo = if current_idx > 0 {
                props.photos.get(current_idx - 1).map(|p| p.to_owned())
            } else {
                None
            };
            let state = props.photo_id.clone();
            let on_click = move |_| {
                if let Some(target_photo) = target_photo.clone() {
                    state.set(Some(target_photo.id));
                }
            };

            html! {
                <Button label="" on_click={on_click}>
                    <IconChevronLeft/>
                </Button>
            }
        };

        let next_button = {
            let target_photo = props.photos.get(current_idx + 1).map(|p| p.to_owned());
            let state = props.photo_id.clone();
            let on_click = move |_| {
                if let Some(target_photo) = target_photo.clone() {
                    state.set(Some(target_photo.id));
                }
            };

            html! {
                <Button label="" on_click={on_click}>
                    <IconChevronRight/>
                </Button>
            }
        };

        let count = {
            html! {
                <span>{current_idx+1}{"/"}{n_photos}</span>
            }
        };

        html! {
            <div class="gallery-detail-overlay">
                <div class="gallery-detail">
                    <div class="header">
                        <DownloadPhotoButton photo_id={photo_id.clone()}/>
                        <PhotoExifButton photo_id={photo_id.clone()}/>
                        <div class="spacing"/>
                        {close_button}
                    </div>
                    <div class="photo">
                        <PhotoPreview photo_id={photo_id}/>
                    </div>
                    <div class="footer">
                        {prev_button}
                        {count}
                        {next_button}
                    </div>
                </div>
            </div>
        }
    } else {
        html! {}
    }
}
