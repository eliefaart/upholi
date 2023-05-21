use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DownloadPhotoButtonProps {
    pub photo_id: AttrValue,
}

#[function_component(DownloadPhotoButton)]
pub fn download_photo_button(_: &DownloadPhotoButtonProps) -> Html {
    // Disabled (render nothing)
    // TODO: Fix this component by making the download actually work.
    html! {}

    // let download_elem_ref = use_node_ref();

    // let on_click = {
    //     let photo_id = props.photo_id.to_string();
    //     let download_elem_ref = download_elem_ref.clone();

    //     move |_| {
    //         let photo_id = photo_id.clone();
    //         let download_elem_ref = download_elem_ref.clone();

    //         //let document = crate::get_document();
    //         //let download_elem = document.create_element("a").expect("Failed to create <a> element");

    //         wasm_bindgen_futures::spawn_local(async move {
    //             if let Some(download_elem) = download_elem_ref.cast::<HtmlElement>() {
    //                 let src = WASM_CLIENT
    //                     .get_photo_image_src(&photo_id, upholi_lib::PhotoVariant::Original)
    //                     .await
    //                     .unwrap();

    //                 download_elem
    //                     .set_attribute("href", &src)
    //                     .expect("Failed to set href attribute");

    //                 let click_event = Event::new("click").expect("Failed to create event");
    //                 download_elem
    //                     .dispatch_event(&click_event)
    //                     .expect("Failed to dispatch click event");
    //             }
    //         });
    //     }
    // };

    // html! {
    //     <>
    //         <a ref={download_elem_ref} download="test.jpg"></a>
    //         <Button label={"Download"} on_click={on_click}>
    //             <IconDownload/>
    //         </Button>
    //     </>
    // }
}
