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
	photo: wasm.ImageUploadInfo
}

class UploadHelper {

	// Temp; encryption key needs to come from elsewhere
	userEncryptionKey: Uint8Array;

	constructor() {
		this.userEncryptionKey = new Uint8Array();
	}

	public init(): void {
		this.userEncryptionKey = wasm.generate_aes256_key();
	}

	public async uploadPhotos(fileList: FileList, progressUpdated: (progress: FileUploadProgress[]) => void): Promise<void> {
		const upholiClient = new wasm.UpholiClient("http://localhost");
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
	async prepareFileForUpload(file: globalThis.File): Promise<wasm.ImageUploadInfo> {
		if (file) {
			const fileBuffer = await file.arrayBuffer();
			const fileBytes = new Uint8Array(fileBuffer);
			const image = new wasm.ImageUploadInfo(fileBytes);

			console.log(image.bytes.byteLength, image.bytesPreview.byteLength, image.bytesThumbnail.byteLength);
			console.log(image.exif);

			// Encryption -> THis should happen within WASM. JS shouldn't have to do any encryption.
			// I won't even have to expose any keys I guess?

			const fileNonce = "452b4dd698de";
			const fileKey = wasm.generate_aes256_key();

			const fileData: FileData = {
				bytes: fileBytes,
				exif: {}
			};
			const fileDataString = JSON.stringify(fileData);
			const fileDataBuffer = Buffer.from(fileDataString, "utf8");

			const encryptedFileBytes = wasm.aes256_encrypt(fileKey, fileNonce, fileDataBuffer);
			const encryptedFileKey = wasm.aes256_encrypt(this.userEncryptionKey, fileNonce, fileKey);

			const _file: File = {
				id: "" + 1,
				nonce: fileNonce,
				data: encryptedFileBytes,
				dataVersion: 1,
				encryptedFileKey,
				sharedKeys: []
			};

			return image;
		}
		else {
			return Promise.reject("");
		}
	}
}

const instance = new UploadHelper();
export default instance;