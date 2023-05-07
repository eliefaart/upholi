use std::rc::Rc;

use crate::components::FileUploadStatus;
use bounce::{Atom, Slice};
use yew::Reducible;
// use std::cell::RefCell;

// const STATE: RefCell<Vec<UploadQueueItem>> = RefCell::new(Vec::<UploadQueueItem>::new());
// const STATE2: RefCell<UseStateHandle<Vec<UploadQueueItem>>> = RefCell::new(use_state(|| Vec::<UploadQueueItem>::new()));

#[derive(Slice, Atom, PartialEq, Default, Clone)]
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
    pub object_url: String,
}

pub enum UploadQueueAction {
    AddItem(UploadQueueItem),
    UpdateItemState {
        file_name: String,
        status: FileUploadStatus,
    },
    // RemoteItem {
    //     file_name: String,
    // },
}

impl Reducible for UploadQueue {
    type Action = UploadQueueAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        weblog::console_log!("lol");
        match action {
            UploadQueueAction::AddItem(item) => {
                let mut updated = self.queue.clone();
                updated.push(item);
                Self { queue: updated }.into()
            }
            UploadQueueAction::UpdateItemState { file_name, status } => Self {
                queue: self.queue.clone(),
            }
            .into(),
        }
    }
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
