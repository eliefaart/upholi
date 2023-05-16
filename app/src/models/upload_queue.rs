use crate::components::FileUploadStatus;
use bounce::Slice;
use std::rc::Rc;
use web_sys::{File, FileList};
use yew::Reducible;

#[derive(Slice, PartialEq, Default, Clone)]
pub struct UploadQueue {
    pub queue: Vec<UploadQueueItem>,
}

impl UploadQueue {
    pub fn items_completed_len(&self) -> usize {
        self.queue
            .iter()
            .filter(|item| item.status == FileUploadStatus::Done || item.status == FileUploadStatus::Exists)
            .count()
    }
}

#[derive(PartialEq, Clone)]
pub struct UploadQueueItem {
    pub filename: String,
    pub size: f64,
    pub status: FileUploadStatus,
    pub file: web_sys::File,
    pub object_url: String,
}

impl From<File> for UploadQueueItem {
    fn from(file: File) -> Self {
        let file_name = file.name();
        let object_url =
            web_sys::Url::create_object_url_with_blob(&file).expect("Failed to create object url from file");

        Self {
            filename: file_name,
            size: file.size(),
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
                        let file: UploadQueueItem = file.into();
                        let already_in_queue = queue
                            .iter()
                            .any(|item| item.filename == file.filename && item.size == file.size);
                        if !already_in_queue {
                            queue.push(file);
                        }
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
