import init, { aes256_encrypt, aes256_decrypt, generate_aes256_key, ImageUploadInfo, test_reqwest } from "wasm";


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

class UploadHelper {

	// Temp; encryption key needs to come from elsewhere
	userEncryptionKey: Uint8Array;

	constructor() {
		this.userEncryptionKey = new Uint8Array();
	}

	public init(): void {
		this.userEncryptionKey = generate_aes256_key();
	}

	/**
	 * Convert a FileList into an array of objects (..)
	 * @param fileList
	 */
	public async prepareFileListForUpload(fileList: FileList): Promise<File[]> {

		const files: File[] = [];

		for (let i = 0; i < fileList.length; i++) {
			const file = fileList.item(i);
			if (file) {
				const fileBuffer = await file.arrayBuffer();
				const fileBytes = new Uint8Array(fileBuffer);
				const fileNonce = "452b4dd698de";
				const fileKey = generate_aes256_key();

				const image = new ImageUploadInfo(fileBytes);

				console.log(image.exifFocalLength);
				console.log(image.exifManufactorer);

				console.log(await test_reqwest());

				const preview_bytes = image.get_preview_bytes();
				const thumbnail_bytes = image.get_thumbnail_bytes();

				const fileData: FileData = {
					bytes: fileBytes,
					exif: {}
				};
				const fileDataString = JSON.stringify(fileData);
				const fileDataBuffer = Buffer.from(fileDataString, "utf8");

				const encryptedFileBytes = aes256_encrypt(fileKey, fileNonce, fileDataBuffer);
				const encryptedFileKey = aes256_encrypt(this.userEncryptionKey, fileNonce, fileKey);

				files.push({
					id: "" + 1,
					nonce: fileNonce,
					data: encryptedFileBytes,
					dataVersion: 1,
					encryptedFileKey,
					sharedKeys: []
				});
			}
		}

		return files;
	}
}

const instance = new UploadHelper();
export default instance;