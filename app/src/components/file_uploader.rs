use crate::{
    components::{FileUploadStatus, UploadProgress},
    hooks::{UploadQueue, UploadQueueAction, UploadQueueItem},
    WASM_CLIENT,
};
use bounce::use_slice;
use js_sys::Uint8Array;
use yew::prelude::*;

#[function_component(FileUploader)]
pub fn file_uploader() -> Html {
    let processing = use_state(Vec::<String>::new);
    let slice_state = use_slice::<UploadQueue>();

    {
        let processing = processing.clone();
        //let queue_rc = slice.clone();
        let slice = slice_state.clone();

        use_effect_with_deps(
            move |_| {
                if !slice.queue.is_empty() {
                    let mut currently_processing = (*processing).clone();
                    let batch: Vec<UploadQueueItem> = slice
                        .queue
                        .clone()
                        .into_iter()
                        .filter(|item| !currently_processing.contains(&item.filename))
                        .map(|item| item.to_owned())
                        .collect();
                    currently_processing.extend(batch.iter().map(|item| item.filename.clone()));
                    processing.set(currently_processing);

                    let set_status = {
                        move |file_name: &str, status: FileUploadStatus| {
                            weblog::console_log!(&format!(":: {file_name} -> {status:?}"));

                            slice.dispatch(UploadQueueAction::UpdateItemState {
                                file_name: file_name.to_string(),
                                status,
                            });
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

                                match WASM_CLIENT.upload_photo(&bytes).await {
                                    Ok(_upload_result) => {
                                        set_status(
                                            &queue_item.filename,
                                            if _upload_result.skipped {
                                                FileUploadStatus::Exists
                                            } else {
                                                FileUploadStatus::Done
                                            },
                                        );
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
            slice_state.queue.clone(),
        );
    }

    html! {
        <UploadProgress/>
    }
}
