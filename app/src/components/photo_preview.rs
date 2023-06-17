use crate::hooks::use_photo_src::use_photo_src;
use upholi_lib::PhotoVariant;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PhotoPreviewProps {
    pub photo_id: AttrValue,
}

#[function_component(PhotoPreview)]
pub fn photo_preview(props: &PhotoPreviewProps) -> Html {
    let src = use_photo_src(&props.photo_id, PhotoVariant::Preview);
    let src = (*src).clone();
    let zoom = use_state(|| 1.0);

    let toggle_zoom = {
        let zoom = zoom.clone();
        move |_| {
            let target_zoom = if (*zoom) == 1.0 { 2.0 } else { 1.0 };
            zoom.set(target_zoom)
        }
    };

    let style = format!("scale: {};", *zoom);
    html! {
        <div class="photo"
            ondblclick={toggle_zoom}>
            <img
                src={src}
                draggable="false"
                style={style}/>
        </div>
    }
}
