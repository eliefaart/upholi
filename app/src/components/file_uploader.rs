use crate::{
    components::{FileUploadStatus, UploadProgress},
    hooks::{UploadQueue, UploadQueueItem},
    WASM_CLIENT,
};
use bounce::{use_atom, use_atom_setter};
use js_sys::Uint8Array;
use yew::prelude::*;

#[function_component(FileUploader)]
pub fn file_uploader() -> Html {
    let upload_queue = use_atom::<UploadQueue>();
    let setter = use_atom_setter::<UploadQueue>();
    let processing = use_state(Vec::<String>::new);

    {
        let processing = processing.clone();

        use_effect_with_deps(
            move |upload_queue| {
                if !upload_queue.is_empty() {
                    let mut currently_processing = (*processing).clone();
                    let batch: Vec<UploadQueueItem> = upload_queue
                        .into_iter()
                        .filter(|item| !currently_processing.contains(&item.filename))
                        .map(|item| item.to_owned())
                        .collect();
                    currently_processing.extend(batch.iter().map(|item| item.filename.clone()));
                    processing.set(currently_processing);

                    let mut set_status = {
                        let mut batch = batch.clone();

                        move |file_name: &str, status: FileUploadStatus| {
                            weblog::console_log!(&format!("{file_name} .. {status:?}"));

                            if let Some(mut queue_item) = batch.iter_mut().find(|item| &item.filename == file_name) {
                                queue_item.status = status;
                                setter(UploadQueue {
                                    queue: batch.to_owned(),
                                })
                            }
                        }
                    };

                    if !batch.is_empty() {
                        wasm_bindgen_futures::spawn_local(async move {
                            weblog::console_log!(&format!("Batch! {}", batch.len()));

                            for queue_item in batch {
                                set_status(&queue_item.filename, FileUploadStatus::Processing);

                                let promise = queue_item.file.array_buffer();
                                let js_value = wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
                                let array = Uint8Array::new(&js_value);
                                let bytes: Vec<u8> = array.to_vec();

                                //set_status(&mut file_progress, FileUploadStatus::Uploading);
                                match WASM_CLIENT.upload_photo(&bytes).await {
                                    Ok(_upload_result) => {
                                        //file_progress.uploaded_photo_id = Some(upload_result.photo_id);
                                        set_status(&queue_item.filename, FileUploadStatus::Done);
                                    }
                                    Err(error) => {
                                        set_status(&queue_item.filename, FileUploadStatus::Failed);
                                        weblog::console_error!(format!("{error:?}"));
                                    }
                                }
                            }
                        });
                    }
                }
            },
            upload_queue.queue.clone(),
        );
    }

    html! {
        <UploadProgress/>
    }
}
