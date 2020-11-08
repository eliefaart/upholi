interface File {
	name: string,
	status: string,
	objectUrl: string
}

export default class UploadHelper {

	constructor() { }

	/**
	 * Convert a FileList to an array of files that can be used as view model for the upload UI
	 * @param fileList
	 */
	static convertFileListToFileArrayForUploadDialog(fileList: FileList) : File[] {
		const files: File[] = [];

		for (let i = 0; i < fileList.length; i++) {
			let file = fileList.item(i);
			if (file) {
				files.push({
					name: file.name,
					status: "",
					objectUrl:URL.createObjectURL(file)
				});
			}
		}

		return files;
	}
}