use crate::components::FileUploadStatus;
use bounce::{Atom, Slice};
use std::rc::Rc;
use web_sys::{File, FileList};
use yew::Reducible;

#[derive(Slice, Atom, PartialEq, Default, Clone)]
pub struct UploadQueue {
    pub queue: Vec<UploadQueueItem>,
}

#[derive(PartialEq, Clone)]
pub struct UploadQueueItem {
    pub filename: String,
    pub status: FileUploadStatus,
    pub file: web_sys::File,
    pub object_url: String,
}

impl From<File> for UploadQueueItem {
    fn from(file: File) -> Self {
        let file_name = file.name().clone();
        let object_url =
            web_sys::Url::create_object_url_with_blob(&file).expect("Failed to create object url from file");

        Self {
            filename: file_name,
            status: FileUploadStatus::Queued,
            file,
            object_url,
        }
    }
}

pub enum UploadQueueAction {
    AddToQueue(FileList),
    UpdateItemState {
        file_name: String,
        status: FileUploadStatus,
    },
    RemoveCompleted,
}

impl Reducible for UploadQueue {
    type Action = UploadQueueAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            UploadQueueAction::AddToQueue(filelist) => {
                let mut queue = self.queue.clone();

                for i in 0..filelist.length() {
                    if let Some(file) = filelist.get(i) {
                        queue.push(file.into());
                    }
                }

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
