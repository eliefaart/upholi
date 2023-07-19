use crate::{
    components::IconUpload,
    models::{UploadQueue, UploadQueueAction},
};
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
    #[prop_or_default]
    pub class: Classes,
    pub children: Children,
    #[prop_or_default]
    pub target_album_id: Option<AttrValue>,
}

#[function_component(DropUpload)]
pub fn drop_upload(props: &DropUploadProps) -> Html {
    let hovering = use_state(|| false);
    let slice = use_slice::<UploadQueue>();

    let on_drop = {
        let target_album_id = props.target_album_id.clone();

        use_callback(
            move |event: DragEvent, hovering| {
                hovering.set(false);
                event.prevent_default();

                if let Some(data_transfer) = event.data_transfer() {
                    if let Some(filelist) = data_transfer.files() {
                        slice.dispatch(UploadQueueAction::AddToQueue {
                            filelist,
                            target_album_id: target_album_id.clone().map(|v| v.to_string()),
                        });
                    }
                }
            },
            hovering.clone(),
        )
    };

    let on_drag_over = use_callback(
        |event: DragEvent, hovering| {
            hovering.set(true);
            event.prevent_default();
        },
        hovering.clone(),
    );
    let on_drag_end = use_callback(|_, hovering| hovering.set(false), hovering.clone());

    html! {
        <div class={classes!(props.class.clone(), "drop-upload")}
            ondrop={on_drop}
            ondragover={on_drag_over}
            ondragleave={on_drag_end}>
            {props.children.clone()}
            if *hovering {
                <div class="hover-overlay">
                    <IconUpload/>
                    <span>{"Drop to upload"}</span>
                </div>
            }
        </div>
    }
}
