export enum FileUploadStatus {
  Queued = "Queued",
  Processing = "Processing",
  Uploading = "Uploading",
  Done = "Done",
  Failed = "Failed",
  Cancelled = "Cancelled",
  Exists = "Exists",
}

export const uploadFinishedStatusses = [
  FileUploadStatus.Done,
  FileUploadStatus.Failed,
  FileUploadStatus.Cancelled,
  FileUploadStatus.Exists,
];

export interface FileUploadProgress {
  file: globalThis.File;
  objectUrl: string;
  status: FileUploadStatus;
  /// ID of photo, returned by server once file has been succesfully uploaded
  uploadedPhotoId: string | null;
}
