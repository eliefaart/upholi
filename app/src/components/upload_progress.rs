use super::FileUploadStatus;
use crate::{
    components::{Button, IconClose},
    models::{UploadQueue, UploadQueueAction},
};
use bounce::use_slice;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UploadProgressProps {}

#[function_component(UploadProgress)]
pub fn upload_progress(_: &UploadProgressProps) -> Html {
    let slice = use_slice::<UploadQueue>();

    let queue_items = slice
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
        let slice = slice.clone();
        move |_| {
            slice.dispatch(UploadQueueAction::RemoveCompleted);
        }
    };

    html! {
        if !slice.queue.is_empty() {
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
