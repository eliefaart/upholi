use crate::components::FileUploadStatus;
use bounce::Atom;
// use std::cell::RefCell;
use yew::prelude::*;

// const STATE: RefCell<Vec<UploadQueueItem>> = RefCell::new(Vec::<UploadQueueItem>::new());
// const STATE2: RefCell<UseStateHandle<Vec<UploadQueueItem>>> = RefCell::new(use_state(|| Vec::<UploadQueueItem>::new()));

#[derive(Atom, PartialEq, Default, Clone)]
pub struct UploadQueue {
    pub queue: Vec<UploadQueueItem>,
}

// impl Default for UploadQueue {
//     fn default() -> Self {
//         UploadQueue { queue: vec![] }
//     }
// }

#[derive(PartialEq, Clone)]
pub struct UploadQueueItem {
    pub filename: String,
    pub status: FileUploadStatus,
    pub file: web_sys::File,
}

// #[hook]
// pub fn use_upload_queue() -> (UseStateHandle<Vec<UploadQueueItem>>, Callback<()>) {
//     let state = use_state(|| STATE.into_inner());
//     // vec![UploadQueueItem {
//     //     filename: "test.jpg".to_string(),
//     //     status: FileUploadStatus::Processing,
//     // }]

//     let update_or_something = { Callback::from(move |_| {}) };

//     (state, update_or_something)
// }
