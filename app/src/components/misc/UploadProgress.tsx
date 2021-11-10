import * as React from "react";
import { FC } from "react";
import { uploadFinishedStatusses } from "../../models/File";
import { IconClose } from "./Icons";
import uploadHelper from "../../helpers/UploadHelper";
import useUploadProgress from "../../hooks/useUploadProgress";

const UploadProgress: FC = () => {
	const uploadProgress = useUploadProgress();
	const queueEmpty = uploadProgress.length === 0;

	if (queueEmpty) {
		return null;
	}
	else {
		const allItemsInQueueFinished = uploadProgress.every(item => uploadFinishedStatusses.indexOf(item.status) !== -1);

		console.log(uploadFinishedStatusses);
		console.log(allItemsInQueueFinished);

		return <div className="uploadProgress">
			<div className="header">
				{allItemsInQueueFinished && <button
					onClick={() => uploadHelper.clearQueue()}
					className="iconOnly">
					<IconClose/>
				</button>}
			</div>
			{uploadProgress.map(file => (
				<div key={file.file.name} className="file">
					<img src={file.objectUrl} className="thumb"/>
					<span className="title">{file.file.name}</span>
					<span className="status">{file.status}</span>
					{/* {file.status === "Failed" && <button>retry</button>} */}
				</div>
			))}
		</div>;
	}
};

export default UploadProgress;