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
    RemoveCompleted, // RemoteItem {
                     //     file_name: String,
                     // },
}

impl Reducible for UploadQueue {
    type Action = UploadQueueAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            UploadQueueAction::AddItem(item) => {
                let mut queue = self.queue.clone();
                queue.push(item);
                Self { queue }.into()
            }
            UploadQueueAction::UpdateItemState { file_name, status } => {
                let mut queue = self.queue.clone();
                if let Some(mut queue_item) = queue.iter_mut().find(|item| item.filename == file_name) {
                    queue_item.status = status;
                }

                Self { queue }.into()
            }
            UploadQueueAction::RemoveCompleted => {
                let mut queue = self.queue.clone();

                queue.retain(|item| item.status != FileUploadStatus::Done && item.status != FileUploadStatus::Exists);

                Self { queue }.into()
            }
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
