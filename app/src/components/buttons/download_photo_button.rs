use crate::{
    components::{buttons::Button, IconDownload},
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
                let src = WASM_CLIENT
                    .get_photo_image_src(&photo_id, upholi_lib::PhotoVariant::Original)
                    .await
                    .unwrap();

                let document = crate::get_document();
                let a_element = document.create_element("a").expect("Failed to create <a> element");
                a_element.set_attribute("href", &src).expect("Failed to set href attribute");
                a_element
                    .set_attribute("download", "test.jpg")
                    .expect("Failed to set download attribute");
                let click_event = Event::new("click").expect("Failed to create event");
                a_element.dispatch_event(&click_event).expect("Failed to dispatch click event");

                weblog::console_log!(format!("-- {:?}", a_element.outer_html()));

                // TODO: Are Event and EventTarget features really needed?
                // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Element.html#method.dispatch_event
            });
        }
    };

    html! {
        <Button label={"Download"} on_click={on_click}>
            <IconDownload/>
        </Button>
    }
}
