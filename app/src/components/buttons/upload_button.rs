use crate::{
    components::{buttons::Button, IconUpload},
    models::{UploadQueue, UploadQueueAction},
};
use bounce::use_slice;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UploadButtonProps {}

#[function_component(UploadButton)]
pub fn upload_button(_: &UploadButtonProps) -> Html {
    let input_ref = use_node_ref();
    let slice = use_slice::<UploadQueue>();

    let on_click = {
        let input_ref = input_ref.clone();
        move |_| {
            if let Some(input_ref) = input_ref.cast::<HtmlInputElement>() {
                input_ref.click();
            }
        }
    };

    let on_change = {
        let input_ref = input_ref.clone();
        move |_| {
            if let Some(input_ref) = input_ref.cast::<HtmlInputElement>() {
                if let Some(filelist) = input_ref.files() {
                    slice.dispatch(UploadQueueAction::AddToQueue(filelist));
                }
            }
        }
    };

    html! {
        <label>
            <Button label={"Upload"} {on_click}>
                <IconUpload/>
            </Button>
            <input id="select-photos"
                ref={input_ref}
                type="file"
                name="photos"
                accept=".jpg,.jpeg"
                onchange={on_change}
                multiple={true} />
        </label>
    }
}
