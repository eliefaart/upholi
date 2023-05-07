use crate::{
    components::{buttons::Button, FileUploadStatus, IconUpload},
    hooks::{UploadQueue, UploadQueueAction, UploadQueueItem},
};
use bounce::{use_atom, use_atom_value, use_slice};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UploadButtonProps {}

#[function_component(UploadButton)]
pub fn upload_button(_: &UploadButtonProps) -> Html {
    let input_ref = use_node_ref();
    let upload_queue = use_atom_value::<UploadQueue>();
    let upload_state = use_atom::<UploadQueue>();
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
                    let mut upload_batch: Vec<UploadQueueItem> = vec![];

                    for i in 0..filelist.length() {
                        if let Some(file) = filelist.get(i) {
                            let file_name = file.name().clone();
                            let object_url = web_sys::Url::create_object_url_with_blob(&file)
                                .expect("Failed to create object url from file");

                            slice.dispatch(UploadQueueAction::AddItem(UploadQueueItem {
                                filename: file_name,
                                status: FileUploadStatus::Queued,
                                file,
                                object_url,
                            }));
                        }
                    }

                    upload_batch.extend(upload_queue.as_ref().clone().queue);
                    upload_state.set(UploadQueue { queue: upload_batch });
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
