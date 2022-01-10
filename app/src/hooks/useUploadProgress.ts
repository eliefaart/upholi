import { useState, useEffect } from "react";
import { FileUploadProgress } from "../models/File";
import uploadHelper from "../helpers/UploadHelper";

export default function useUploadProgress(): FileUploadProgress[] {
	const [progress, setUploadProgress] = useState<FileUploadProgress[]>([]);

	useEffect(() => {
		uploadHelper.subscribe({
			update: setUploadProgress
		});
	}, []);

	return progress;
}