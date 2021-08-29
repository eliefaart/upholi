import { FileUploadProgress, FileUploadStatus } from "../models/File";
import upholiService from "../services/UpholiService";

class UploadHelper {

	public async uploadPhotos(fileList: FileList, progressUpdated: (progress: FileUploadProgress[]) => void): Promise<void> {
		const queue: FileUploadProgress[] = [];

		const updateQueueItemStatus = (item: FileUploadProgress, status: FileUploadStatus) => {
			item.status = status;
			if (progressUpdated) {
				progressUpdated(queue);
			}
		};

		// Create an upload queue from FileList
		for (let i = 0; i < fileList.length; i++) {
			const file = fileList.item(i);
			if (file) {
				queue.push({
					file,
					status: FileUploadStatus.Queued,
					objectUrl: URL.createObjectURL(file)
				});
			}
		}

		if (progressUpdated) {
			progressUpdated(queue);
		}

		// Upload all items in queue
		for (const queueItem of queue) {
			try {
				updateQueueItemStatus(queueItem, FileUploadStatus.Uploading);
				const photoBytes = await queueItem.file.arrayBuffer();

				// TODO: Part of this call seems to block UI updates. Not sure what, since it's async
				await upholiService.uploadPhoto(new Uint8Array(photoBytes));

				updateQueueItemStatus(queueItem, FileUploadStatus.Done);
			}
			catch {
				updateQueueItemStatus(queueItem, FileUploadStatus.Failed);
			}
		}
	}
}

const instance = new UploadHelper();
export default instance;