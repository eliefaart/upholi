use super::FileUploadStatus;
use crate::{
    components::{Button, IconClose},
    hooks::use_upload_queue,
};
use bounce::{use_atom_setter, use_atom_value};
use use_upload_queue::UploadQueue;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UploadProgressProps {}

#[function_component(UploadProgress)]
pub fn upload_progress(_: &UploadProgressProps) -> Html {
    let upload_queue_setter = use_atom_setter::<UploadQueue>();
    let queue = use_atom_value::<UploadQueue>();
    let queue_items = queue
        .as_ref()
        .queue
        .iter()
        .map(|queue_item| {
            html! {
                <UploadProgressItem
                    filename={queue_item.filename.clone()}
                    status={queue_item.status.clone()}
                    object_url={queue_item.object_url.clone()}
                />
            }
        })
        .collect::<Html>();

    let clear_completed = {
        let queue = queue.clone();
        move |_| {
            let items_not_done = queue
                .queue
                .clone()
                .into_iter()
                .filter(|item| item.status != FileUploadStatus::Done)
                .collect();
            upload_queue_setter(UploadQueue { queue: items_not_done });
        }
    };

    html! {
        if !queue.queue.is_empty() {
            <div class="upload-progress">
                <div class="upload-progress-header">
                    <Button label={""} on_click={clear_completed}>
                        <IconClose/>
                    </Button>
                </div>
                <div class="upload-progress-items">
                    {queue_items}
                </div>
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct UploadProgressItemProps {
    pub filename: AttrValue,
    pub status: FileUploadStatus,
    pub object_url: AttrValue,
}

#[function_component(UploadProgressItem)]
pub fn upload_progress_item(props: &UploadProgressItemProps) -> Html {
    html! {
        <div class="upload-progress-item">
            <img class="thumb" src={&props.object_url} />
            <span class="filename">{&props.filename}</span>
            <span class="status">{&props.status}</span>
        </div>
    }
}
