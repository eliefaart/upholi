use crate::{components::FileUploadStatus, models::UploadQueue};
use bounce::use_slice_value;
use yew::prelude::*;

pub struct FileStatus {
    pub filename: String,
    pub status: FileUploadStatus,
}

#[hook]
pub fn use_on_file_upload_finished(callback: Callback<Vec<FileStatus>>) {
    let slice = use_slice_value::<UploadQueue>();
    let notified: UseStateHandle<Vec<String>> =
        use_state(|| slice.queue.iter().map(|qi| qi.filename.to_string()).collect());

    use_effect_with_deps(
        move |slice| {
            let queue_items_not_notified: Vec<FileStatus> = slice
                .queue
                .iter()
                .filter(|qi| {
                    (matches!(qi.status, FileUploadStatus::Done { .. })
                        || matches!(qi.status, FileUploadStatus::Exists { .. }))
                        && !(*notified).contains(&qi.filename)
                })
                .map(|qi| FileStatus {
                    filename: qi.filename.to_string(),
                    status: qi.status.clone(),
                })
                .collect();

            if !queue_items_not_notified.is_empty() {
                let mut just_notified = queue_items_not_notified
                    .iter()
                    .map(|qi| qi.filename.to_string())
                    .collect();
                let mut notified_updated = (*notified).clone();
                notified_updated.append(&mut just_notified);

                callback.emit(queue_items_not_notified);

                notified.set(notified_updated);
            }
        },
        slice,
    );
}
