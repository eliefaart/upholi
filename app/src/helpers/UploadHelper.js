class UploadHelper {

	constructor() {
	}

	static convertFileListToFileArrayForUploadDialog(fileList) {
		return [...fileList].map(file => {
			return {
				name: file.name,
				status: "",
				objectUrl: URL.createObjectURL(file)
			};
		})
	}
}

export default UploadHelper;