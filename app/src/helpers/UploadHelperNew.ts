import * as wasm from "wasm";


/**
 * TODO: Just make this a class, and add functions like 'create new shared key', 'get (unencrypted) file bytes/data'
 */
interface File {
	id: string,
	/**
	 * One per file, or one per key/shared-key?
	 */
	nonce: string,
	/**
	 * Encrypted data. Encrypted using this File's 'file key'.
	 * When decrypted it will be in form 'FileData'
	 */
	data: Uint8Array | FileData,

	/**
	 * Maybe???
	 * If I want to change 'FileData' in the future, then I can't perform a database update.
	 * So older versions of 'data' will linger around for a long time. So I will probably need to keep track of what version of 'data' this file is currently using.
	 */
	dataVersion: number,

	/**
	 * The 'file key', encrypted by the owner's encryption key.
	 */
	encryptedFileKey: Uint8Array,

	sharedKeys: FileSharedKey[]
}

interface FileData {
	bytes: Uint8Array,
	// more misc fields
	exif: {
		// .. aperture, focal length, etc.
	}
}

interface FileSharedKey {
	/**
	 * ID of a share. A share is a public url, or something another user has been given access to.
	 */
	shareId: string,
	/**
	 * The 'file key', encrypted by the share's encryption key.
	 * If the share is shared with another user, this field was encrypted using that user's encryption key.
	 * If the share is a public url, the encryption key can be reconstructed from the url. (todo: decide how)
	 */
	keyEncrypted: Uint8Array,
}

enum FileUploadStatus {
	Queued = "Queued",
	Processing = "Processing",
	Uploading = "Uploading",
	Done = "Done",
	Failed = "Failed"
}
interface FileUploadProgress {
	file: globalThis.File,
	status: FileUploadStatus
}
interface FileUploadQueueItem extends FileUploadProgress {
	photo: wasm.PhotoUploadInfo
}

class UploadHelper {

	public async uploadPhotos(fileList: FileList, progressUpdated: (progress: FileUploadProgress[]) => void): Promise<void> {
		const upholiClient = new wasm.UpholiClient("http://localhost", "e0ca4c29d5504e8daa8c52e873e66f71");
		const queue: FileUploadQueueItem[] = [];

		const updateQueueItemStatus = (item: FileUploadQueueItem, status: FileUploadStatus) => {
			item.status = status;
			progressUpdated(queue);
		};

		// Create an upload queue from FileList
		for (let i = 0; i < fileList.length; i++) {
			const file = fileList.item(i);
			if (file) {
				const photo = await this.prepareFileForUpload(file);

				queue.push({
					file,
					status: FileUploadStatus.Queued,
					photo
				});
			}
		}

		// Upload all items in queue
		progressUpdated(queue);
		for (const queueItem of queue) {
			updateQueueItemStatus(queueItem, FileUploadStatus.Uploading);

			await upholiClient.uploadPhoto(queueItem.photo);

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

			console.log(image.bytes.byteLength, image.bytesPreview.byteLength, image.bytesThumbnail.byteLength);
			console.log(image.exif);

			return image;
		}
		else {
			return Promise.reject("");
		}
	}
}

const instance = new UploadHelper();
export default instance;