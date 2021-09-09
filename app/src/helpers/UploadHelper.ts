import { FileUploadProgress, FileUploadStatus } from "../models/File";
import upholiService from "../services/UpholiService";

interface FileUploadProgressObserver {
	update: (progress: FileUploadProgress[]) => void
}

class UploadHelper {

	private uploadQueue: FileUploadProgress[];
	private observers: FileUploadProgressObserver[];

	constructor() {
		this.uploadQueue = [];
		this.observers = [];
	}

	/**
	 * Removes all completed items from the queue
	 */
	public clearQueue() {
		const finishedStatusses = [
			FileUploadStatus.Done,
			FileUploadStatus.Failed,
			FileUploadStatus.Cancelled
		];
		this.uploadQueue = this.uploadQueue.filter(item => finishedStatusses.indexOf(item.status) === -1);
		this.notifyObservers();
	}

	public async uploadPhotos(fileList: FileList): Promise<FileUploadProgress[]> {
		// subQueue holds the current set of files to upload, so caller can respond on the Promise completing for given 'fileList'
		const subQueue: FileUploadProgress[] = [];

		// Create an upload queue from FileList
		for (let i = 0; i < fileList.length; i++) {
			const file = fileList.item(i);
			if (file) {
				const queueItem = {
					file,
					status: FileUploadStatus.Queued,
					objectUrl: URL.createObjectURL(file),
					uploadedPhotoId: null
				};

				subQueue.push(queueItem);
				this.uploadQueue.push(queueItem);
			}
		}

		this.notifyObservers();

		// Upload all items in queue
		for (const queueItem of subQueue) {
			// Check if item is still in main queue
			const stillQueued = this.uploadQueue.indexOf(queueItem) !== -1;
			if (stillQueued) {
				try {
					this.updateQueueItemStatus(queueItem, FileUploadStatus.Uploading);
					const photoBytes = await queueItem.file.arrayBuffer();

					// TODO: Part of this call seems to block UI updates. Not sure what, since it's async
					const photoId = await upholiService.uploadPhoto(new Uint8Array(photoBytes));
					queueItem.uploadedPhotoId = photoId;

					this.updateQueueItemStatus(queueItem, FileUploadStatus.Done);
				}
				catch {
					this.updateQueueItemStatus(queueItem, FileUploadStatus.Failed);
				}
			}
		}

		return subQueue;
	}

	/**
	 * Update the status of one of the items in the queue
	 */
	private updateQueueItemStatus(item: FileUploadProgress, status: FileUploadStatus) {
		item.status = status;
		this.notifyObservers();
	}

	/**
	 * Subscribe to changes in the upload queue
	 */
	public subscribe(observer: FileUploadProgressObserver) {
		this.observers.push(observer);
	}

	public unsubscribe(observer: FileUploadProgressObserver) {
		this.observers = this.observers.filter(ob => ob !== observer);
	}

	private notifyObservers() {
		for (const observer of this.observers) {
			observer.update(this.uploadQueue);
		}
	}
}

const instance = new UploadHelper();
export default instance;