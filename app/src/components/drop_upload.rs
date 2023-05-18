use crate::models::{UploadQueue, UploadQueueAction};
use bounce::use_slice;
use std::fmt::Display;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum FileUploadStatus {
    Queued,
    Busy,
    Done { photo_id: String },
    Failed,
    Exists { photo_id: String },
}

impl Display for FileUploadStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Done { .. } => write!(f, "Done"),
            Self::Exists { .. } => write!(f, "Exists"),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FileUploadProgress {
    pub file_name: String,
    pub status: FileUploadStatus,
    pub uploaded_photo_id: Option<String>,
}

#[derive(Properties, PartialEq)]
pub struct DropUploadProps {
    pub children: Children,
    pub on_upload_status_changed: Callback<FileUploadProgress>,
}

#[function_component(DropUpload)]
pub fn drop_upload(props: &DropUploadProps) -> Html {
    let slice = use_slice::<UploadQueue>();
    let on_drop = move |event: DragEvent| {
        event.prevent_default();

        if let Some(data_transfer) = event.data_transfer() {
            if let Some(filelist) = data_transfer.files() {
                slice.dispatch(UploadQueueAction::AddToQueue(filelist));
            }
        }
    };

    html! {
        <div style="height: 95%"
            ondrop={on_drop}
            ondragover={|event: DragEvent| event.prevent_default()}>
            {props.children.clone()}
        </div>
    }
}
