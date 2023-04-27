use crate::{
    components::UploadProgress,
    hooks::{UploadQueue, UploadQueueItem},
};
use bounce::{use_atom, use_atom_value};
use crossbeam::channel::bounded;
use yew::prelude::*;
use yew_hooks::use_effect_once;

#[function_component(FileUploader)]
pub fn file_uploader() -> Html {
    let upload_queue = use_atom_value::<UploadQueue>();
    let channel = use_state(|| bounded::<UploadQueueItem>(2));

    {
        let sender = channel.0.clone();
        use_effect_with_deps(
            move |upload_queue| {
                weblog::console_log!(upload_queue.as_ref().queue.len());

                for item in &upload_queue.queue {
                    if let Err(error) = sender.send(item.to_owned()) {
                        weblog::console_error!(format!("{:?}", error));
                    }
                }
            },
            upload_queue,
        );
    }

    {
        use_effect_once(move || {
            let receiver = channel.1.clone();
            wasm_bindgen_futures::spawn_local(async move {
                weblog::console_log!("#1");
                // loop {
                //     thread::sleep(Duration::from_secs(1));

                // }

                // loop {
                //     // match receiver.try_recv() {
                //     //     Ok(item) => {
                //     //         weblog::console_log!("abc");
                //     //         weblog::console_log!(item.filename);
                //     //     }
                //     //     Err(error) => weblog::console_error!(format!("{:?}", error)),
                //     // }
                // }

                // match receiver.try_recv() {
                //     Ok(item) => {
                //         weblog::console_log!("abc");
                //         weblog::console_log!(item.filename);
                //     }
                //     Err(error) => weblog::console_error!(format!("{:?}", error)),
                // }

                match receiver.recv_timeout(std::time::Duration::from_secs(10)) {
                    Ok(item) => {
                        weblog::console_log!("abc");
                        weblog::console_log!(item.filename);
                    }
                    Err(error) => weblog::console_error!(format!("{:?}", error)),
                }

                // for item in receiver.recv_timeout(std::time::Duration::from_secs(10)) {
                //     weblog::console_log!("abc");
                //     weblog::console_log!(item.filename.clone());
                // }
                weblog::console_log!("#2");
            });
            || {}
        })
    }

    html! {
        <UploadProgress/>
    }
}
