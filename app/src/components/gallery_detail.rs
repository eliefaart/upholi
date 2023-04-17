use crate::{
    components::{Button, DownloadPhotoButton, IconClose, PhotoInfoButton, PhotoPreview},
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
        let close_button = {
            let photo_id = props.photo_id.clone();
            html! {
                <Button label="" on_click={move |_| photo_id.set(None) }>
                    <IconClose/>
                </Button>
            }
        };

        html! {
            <div class="gallery-detail-overlay">
                <div class="gallery-detail">
                    <div class="header">
                        <PhotoInfoButton photo_id={photo_id.clone()}/>
                        <DownloadPhotoButton photo_id={photo_id.clone()}/>
                        <div class="spacing"/>
                        {close_button}
                    </div>
                    <div class="photo">
                        <PhotoPreview photo_id={photo_id}/>
                    </div>
                </div>
            </div>
        }
    } else {
        html! {}
    }
}
