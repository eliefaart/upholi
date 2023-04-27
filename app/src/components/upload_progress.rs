use crate::hooks::use_upload_queue;
use bounce::use_atom_value;
use use_upload_queue::UploadQueue;
use yew::prelude::*;

use super::FileUploadStatus;

#[derive(Properties, PartialEq)]
pub struct UploadProgressProps {}

#[function_component(UploadProgress)]
pub fn upload_progress(_: &UploadProgressProps) -> Html {
    let queue = use_atom_value::<UploadQueue>();
    let queue_items = queue
        .as_ref()
        .queue
        .iter()
        .map(|queue_item| {
            html! {
                <UploadProgressItem filename={queue_item.filename.clone()} status={queue_item.status.clone()}/>
            }
        })
        .collect::<Html>();

    html! {
        <div class="upload-progress">
            {queue_items}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct UploadProgressItemProps {
    pub filename: AttrValue,
    pub status: FileUploadStatus,
}

#[function_component(UploadProgressItem)]
pub fn upload_progress_item(props: &UploadProgressItemProps) -> Html {
    html! {
        <div class="upload-progress-item">
            <span class="filename">{&props.filename}</span>
            <span class="status">{&props.status}</span>
        </div>
    }
}
