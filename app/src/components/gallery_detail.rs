use crate::{
    components::{
        Button, DownloadPhotoButton, IconChevronLeft, IconChevronRight, IconClose, PhotoExifButton, PhotoPreview,
    },
    models::AlbumPhoto,
};
use yew::prelude::*;
use yew_router::prelude::{use_navigator, use_route};

#[derive(PartialEq, Properties)]
pub struct GalleryDetailProps {
    pub photos: Vec<AlbumPhoto>,
    pub photo_id: AttrValue,
}

#[function_component(GalleryDetail)]
pub fn gallery_detail(props: &GalleryDetailProps) -> Html {
    let navigator = use_navigator().unwrap();
    let route = use_route::<crate::Route>().unwrap();

    let n_photos = props.photos.len();
    let current_idx = props
        .photos
        .iter()
        .position(|p| props.photo_id.eq(&p.id))
        .unwrap_or_default();

    let close_button = {
        let navigator = navigator.clone();

        html! {
            <Button label="" on_click={move |_| navigator.back() }>
                <IconClose/>
            </Button>
        }
    };

    let prev_button = {
        let navigator = navigator.clone();
        let route = route.clone();

        let target_photo = if current_idx > 0 {
            props.photos.get(current_idx - 1).map(|p| p.to_owned())
        } else {
            None
        };
        let on_click = move |_| {
            let navigator = navigator.clone();
            let route = route.clone();

            if let Some(target_photo) = target_photo.clone() {
                navigator.replace_with_state(&route, target_photo.id);
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

        let on_click = move |_| {
            let navigator = navigator.clone();
            let route = route.clone();

            if let Some(target_photo) = target_photo.clone() {
                navigator.replace_with_state(&route, target_photo.id);
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
                    <DownloadPhotoButton photo_id={&props.photo_id}/>
                    <PhotoExifButton photo_id={&props.photo_id}/>
                    <div class="spacing"/>
                    {close_button}
                </div>
                <div class="photo">
                    <PhotoPreview photo_id={&props.photo_id}/>
                </div>
                <div class="footer">
                    {prev_button}
                    {count}
                    {next_button}
                </div>
            </div>
        </div>
    }
}
