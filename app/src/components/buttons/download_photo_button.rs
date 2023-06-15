use crate::{
    components::{Button, IconDownload},
    WASM_CLIENT,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DownloadPhotoButtonProps {
    pub photo_id: AttrValue,
}

#[function_component(DownloadPhotoButton)]
pub fn download_photo_button(props: &DownloadPhotoButtonProps) -> Html {
    let on_click = {
        let photo_id = props.photo_id.to_string();

        move |_| {
            let photo_id = photo_id.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let filename = format!("{photo_id}.jpg");
                let src = WASM_CLIENT
                    .get_photo_image_src(&photo_id, upholi_lib::PhotoVariant::Original)
                    .await
                    .unwrap();

                // Offering the file as download happens in a JavaScript function which we call from here.
                // This is because I couldn't get it to work from here.
                crate::offer_as_file_download(&filename, &src);
            });
        }
    };

    html! {
        <Button label={"Download"} on_click={on_click}>
            <IconDownload/>
        </Button>
    }
}
