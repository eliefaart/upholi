use crate::{
    components::{buttons::Button, IconInfo, PhotoExif},
    hooks::use_photo,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PhotoExifButtonProps {
    pub photo_id: AttrValue,
}

#[function_component(PhotoExifButton)]
pub fn photo_exif_button(props: &PhotoExifButtonProps) -> Html {
    let photo = use_photo(props.photo_id.as_str());
    let visible = use_state(|| false);

    let on_click = {
        let visible = visible.clone();
        move |_| {
            visible.set(!*visible);
        }
    };

    let exif = (*photo).clone().and_then(|p| p.exif);
    if let Some(exif) = exif {
        let exif_container = if *visible {
            html! {
                <div class="photo-exif-wrapper">
                    <PhotoExif exif={exif}/>
                </div>
            }
        } else {
            html! {}
        };

        html! {
            <>
                <Button label={"Exif"} on_click={on_click}>
                    <IconInfo/>
                </Button>
                {exif_container}
            </>
        }
    } else {
        html! {}
    }
}
