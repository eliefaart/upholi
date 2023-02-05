import * as React from "react";
import { FC } from "react";
import { FileUploadStatus, uploadFinishedStatusses } from "../../models/File";
import { IconClose } from "./Icons";
import uploadHelper from "../../helpers/UploadHelper";
import useUploadProgress from "../../hooks/useUploadProgress";
import Button from "./Button";

const UploadProgress: FC = () => {
	const [queueFinished, setQueueFinished] = React.useState(false);
	const [queueEmpty, setQueueEmpty] = React.useState(false);
	const uploadProgress = useUploadProgress();

	if (queueFinished !== uploadProgress.every(item => uploadFinishedStatusses.indexOf(item.status) !== -1)) {
		setQueueFinished(!queueFinished);
	}

	if (queueEmpty !== (uploadProgress.length === 0)) {
		setQueueEmpty(!queueEmpty);
	}

	if (queueEmpty) {
		return null;
	}
	else {
		const nTotal = uploadProgress.length;
		const nDone = uploadProgress.filter(p => p.status === FileUploadStatus.Done).length;

		return <div className="upload-progress">
			<div className="header">
				<span className="title">Uploaded {nDone}/{nTotal}</span>
				<div className="actions">
					{queueFinished && <Button
						onClick={() => uploadHelper.clearQueue()}
						label=""
						icon={<IconClose />} />}
				</div>
			</div>
			<div className="files">
				{uploadProgress.map(file => (
					<div key={file.file.name} className="file">
						<img src={file.objectUrl} className="thumb" />
						<span className="title">{file.file.name}</span>
						<span className="status">{file.status}</span>
					</div>
				))}
			</div>
		</div>;
	}
};

export default UploadProgress;