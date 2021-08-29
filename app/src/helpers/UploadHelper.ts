import * as wasm from "wasm";
import { FileUploadProgress, FileUploadStatus } from "../models/File";

class UploadHelper {

	public async uploadPhotos(fileList: FileList, progressUpdated: (progress: FileUploadProgress[]) => void): Promise<void> {
		const upholiClient = new wasm.UpholiClient("http://localhost", "e0ca4c29d5504e8daa8c52e873e66f71");

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

		// Upload all items in queue
		if (progressUpdated) {
			progressUpdated(queue);
		}
		for (const queueItem of queue) {
			// TODO: UI hangs on this call it seems
			updateQueueItemStatus(queueItem, FileUploadStatus.Processing);
			const photo = await this.prepareFileForUpload(queueItem.file);

			updateQueueItemStatus(queueItem, FileUploadStatus.Uploading);
			await upholiClient.uploadPhoto(photo);

			updateQueueItemStatus(queueItem, FileUploadStatus.Done);
		}
	}

	/**
	 * Convert a File into an object that can be uploaded to server.
	 * @param fileList
	 */
	async prepareFileForUpload(file: globalThis.File): Promise<wasm.PhotoUploadInfo> {
		if (file) {
			const fileBuffer = await file.arrayBuffer();
			const fileBytes = new Uint8Array(fileBuffer);
			const image = new wasm.PhotoUploadInfo(fileBytes);

			return image;
		}
		else {
			return Promise.reject("");
		}
	}
}

const instance = new UploadHelper();
export default instance;