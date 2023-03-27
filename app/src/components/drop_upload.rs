use crate::WASM_CLIENT;
use js_sys::Uint8Array;
use weblog::{console_error, console_log};
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum FileUploadStatus {
    Queued,
    Processing,
    Uploading,
    Done,
    Failed,
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
    let on_upload_status_changed = props.on_upload_status_changed.clone();
    let on_drop = move |event: DragEvent| {
        event.prevent_default();

        if let Some(data_transfer) = event.data_transfer() {
            if let Some(filelist) = data_transfer.files() {
                let on_upload_status_changed = on_upload_status_changed.clone();
                let set_status = move |progress: &mut FileUploadProgress, status: FileUploadStatus| {
                    progress.status = status;
                    on_upload_status_changed.emit(progress.clone());
                };

                wasm_bindgen_futures::spawn_local(async move {
                    let mut files: Vec<(web_sys::File, FileUploadProgress)> = vec![];

                    //	Initialize the queue
                    for i in 0..filelist.length() {
                        if let Some(file) = filelist.get(i) {
                            let file_name = file.name().clone();
                            let mut progress = FileUploadProgress {
                                file_name,
                                status: FileUploadStatus::Queued,
                                uploaded_photo_id: None,
                            };

                            set_status(&mut progress, FileUploadStatus::Queued);

                            files.push((file, progress));
                        }
                    }

                    // Upload files in the queue
                    for (file, mut file_progress) in files {
                        set_status(&mut file_progress, FileUploadStatus::Processing);

                        let promise = file.array_buffer();
                        let js_value = wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
                        let array = Uint8Array::new(&js_value);
                        let bytes: Vec<u8> = array.to_vec();

                        set_status(&mut file_progress, FileUploadStatus::Uploading);
                        match WASM_CLIENT.upload_photo(&bytes).await {
                            Ok(upload_result) => {
                                file_progress.uploaded_photo_id = Some(upload_result.photo_id);
                                set_status(&mut file_progress, FileUploadStatus::Done);
                            }
                            Err(error) => {
                                set_status(&mut file_progress, FileUploadStatus::Failed);
                                console_error!(format!("{error:?}"));
                            }
                        }
                    }
                });
            }
        }
    };

    html! {
        <div style="height: 100%;"
            ondrop={on_drop}
            ondragover={|event: DragEvent| event.prevent_default()}>
            {props.children.clone()}
        </div>
    }
}
