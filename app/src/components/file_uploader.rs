use crate::{
    components::{FileUploadStatus, UploadProgress},
    models::{UploadQueue, UploadQueueAction, UploadQueueItem},
    WASM_CLIENT,
};
use bounce::use_slice;
use js_sys::Uint8Array;
use weblog::console_error;
use yew::prelude::*;

#[function_component(FileUploader)]
pub fn file_uploader() -> Html {
    let processing = use_state(Vec::<String>::new);
    let slice_state = use_slice::<UploadQueue>();

    {
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
                        .collect();
                    currently_processing.extend(batch.iter().map(|item| item.filename.clone()));
                    processing.set(currently_processing);

                    let set_status = {
                        move |file_name: &str, status: FileUploadStatus| {
                            slice.dispatch(UploadQueueAction::UpdateItemState {
                                file_name: file_name.to_string(),
                                status,
                            });
                        }
                    };

                    if !batch.is_empty() {
                        wasm_bindgen_futures::spawn_local(async move {
                            for queue_item in batch {
                                set_status(&queue_item.filename, FileUploadStatus::Busy);

                                let promise = queue_item.file.array_buffer();
                                let js_value = wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
                                let array = Uint8Array::new(&js_value);
                                let bytes: Vec<u8> = array.to_vec();

                                match WASM_CLIENT.upload_photo(&bytes).await {
                                    Ok(upload_result) => {
                                        if let Some(album_id) = queue_item.target_album_id {
                                            let result = WASM_CLIENT
                                                .add_photos_to_album(&album_id, &[upload_result.photo_id.clone()])
                                                .await;
                                            if let Err(error) = result {
                                                console_error!(format!("{error}"));
                                            }
                                        }

                                        set_status(
                                            &queue_item.filename,
                                            if upload_result.skipped {
                                                FileUploadStatus::Exists {
                                                    photo_id: upload_result.photo_id,
                                                }
                                            } else {
                                                FileUploadStatus::Done {
                                                    photo_id: upload_result.photo_id,
                                                }
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
