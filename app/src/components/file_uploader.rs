use crate::{
    components::{FileUploadStatus, UploadProgress},
    hooks::{UploadQueue, UploadQueueAction, UploadQueueItem},
    WASM_CLIENT,
};
use bounce::{use_atom, use_atom_setter, use_atom_value, use_slice};
use js_sys::Uint8Array;
use yew::prelude::*;

#[function_component(FileUploader)]
pub fn file_uploader() -> Html {
    //let upload_queue = use_atom::<UploadQueue>();
    // let queue_value = use_atom_value::<UploadQueue>();
    // let queue_setter = use_atom_setter::<UploadQueue>();
    let processing = use_state(Vec::<String>::new);
    let slice_state = use_slice::<UploadQueue>();

    weblog::console_log!("file_uploader");

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
                        //let queue_rc = queue_rc.clone();
                        weblog::console_log!("ok");

                        move |file_name: &str, status: FileUploadStatus| {
                            weblog::console_log!(&format!(":: {file_name} -> {status:?}"));

                            slice.dispatch(UploadQueueAction::UpdateItemState {
                                file_name: file_name.to_string(),
                                status,
                            });

                            // if let Some(mut queue_item) = queue_rc
                            //     .queue
                            //     .clone()
                            //     .iter_mut()
                            //     .find(|item| &item.filename == file_name)
                            // {
                            //     slice.dispatch(UploadQueueAction::UpdateItemState {
                            //         file_name: file_name.to_string(),
                            //         status,
                            //     });
                            //     // queue_item.status = status;
                            //     // queue_setter(UploadQueue {
                            //     //     queue: queue_rc.queue.to_owned(),
                            //     // })
                            // }
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

    // {
    //     let processing = processing.clone();
    //     let queue_rc = queue_value.clone();

    //     use_effect_with_deps(
    //         move |_| {
    //             if !queue_rc.queue.is_empty() {
    //                 let mut currently_processing = (*processing).clone();
    //                 let batch: Vec<UploadQueueItem> = queue_rc
    //                     .queue
    //                     .clone()
    //                     .into_iter()
    //                     .filter(|item| !currently_processing.contains(&item.filename))
    //                     .map(|item| item.to_owned())
    //                     .collect();
    //                 currently_processing.extend(batch.iter().map(|item| item.filename.clone()));
    //                 processing.set(currently_processing);

    //                 let set_status = {
    //                     let queue_rc = queue_rc.clone();
    //                     weblog::console_log!("ok");

    //                     move |file_name: &str, status: FileUploadStatus| {
    //                         weblog::console_log!(&format!(":: {file_name} -> {status:?}"));

    //                         if let Some(mut queue_item) = queue_rc
    //                             .queue
    //                             .clone()
    //                             .iter_mut()
    //                             .find(|item| &item.filename == file_name)
    //                         {
    //                             queue_item.status = status;
    //                             queue_setter(UploadQueue {
    //                                 queue: queue_rc.queue.to_owned(),
    //                             })
    //                         }
    //                     }
    //                 };

    //                 if !batch.is_empty() {
    //                     wasm_bindgen_futures::spawn_local(async move {
    //                         weblog::console_log!(&format!("Batch! {}", batch.len()));

    //                         for queue_item in batch {
    //                             set_status(&queue_item.filename, FileUploadStatus::Processing);

    //                             let promise = queue_item.file.array_buffer();
    //                             let js_value = wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
    //                             let array = Uint8Array::new(&js_value);
    //                             let bytes: Vec<u8> = array.to_vec();

    //                             //set_status(&mut file_progress, FileUploadStatus::Uploading);
    //                             match WASM_CLIENT.upload_photo(&bytes).await {
    //                                 Ok(_upload_result) => {
    //                                     //file_progress.uploaded_photo_id = Some(upload_result.photo_id);

    //                                     set_status(
    //                                         &queue_item.filename,
    //                                         if _upload_result.skipped {
    //                                             FileUploadStatus::Exists
    //                                         } else {
    //                                             FileUploadStatus::Done
    //                                         },
    //                                     );
    //                                 }
    //                                 Err(error) => {
    //                                     set_status(&queue_item.filename, FileUploadStatus::Failed);
    //                                     weblog::console_error!(format!("{error:?}"));
    //                                 }
    //                             }
    //                         }
    //                     });
    //                 }
    //             }
    //         },
    //         queue_value.clone(),
    //     );
    // }

    html! {
        <UploadProgress/>
    }
}
