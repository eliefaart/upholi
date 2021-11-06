import { useState } from "react";
import { FileUploadProgress } from "../models/File";
import uploadHelper from "../helpers/UploadHelper";

export default function useUploadProgress(): FileUploadProgress[] {
	const [progress, setUploadProgress] = useState<FileUploadProgress[]>([]);

	uploadHelper.subscribe({
		update: setUploadProgress
	});

	return progress;
}