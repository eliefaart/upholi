import PhotoService from "../services/PhotoService.js"

class UploadHelper {

	constructor(props) {
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

	// static uploadFileList(component, fileList, fnOnUploadFinished) {
	// 	let componentStateFiles = UploadHelper.convertFileListToFileArrayForUploadDialog(fileList);

	// 	// let fnOnUploadFinished = (uploadedPhotoIds) => {
	// 	// 	component.setState({
	// 	// 		uploadInProgress: false,
	// 	// 		uploadFiles: []
	// 	// 	});
	// 	// };
	// 	let fnUpdateFileUploadState = (file, newState) => {
	// 		let stateFile = componentStateFiles.find(f => f.name === file.name);
	// 		stateFile.status = newState;

	// 		component.setState({
	// 			uploadFiles: componentStateFiles
	// 		});
	// 	};

	// 	PhotoService.uploadPhotos2(fileList, fnUpdateFileUploadState).then((uploadedPhotoIds) => {
	// 		fnOnUploadFinished(uploadedPhotoIds);
	// 	}).catch((error) => {
	// 		console.log(error);
	// 	});

	// 	component.setState({
	// 		uploadInProgress: true,
	// 		uploadFiles: componentStateFiles
	// 	});
	// }
}

export default UploadHelper;