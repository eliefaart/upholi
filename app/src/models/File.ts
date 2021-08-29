export enum FileUploadStatus {
	Queued = "Queued",
	Processing = "Processing",
	Uploading = "Uploading",
	Done = "Done",
	Failed = "Failed"
}

export interface FileUploadProgress {
	file: globalThis.File,
	objectUrl: string,
	status: FileUploadStatus
}