import * as React from "react";
import { FileUploadProgress, FileUploadStatus } from "../models/File";
import { IconClose } from "./Icons";
import uploadHelper from "../helpers/UploadHelper";

interface Props {
	progress: FileUploadProgress[]
}

class UploadProgress extends React.Component<Props> {

	constructor(props: Props) {
		super(props);

		this.close = this.close.bind(this);
	}

	close(): void {
		uploadHelper.clearQueue();
	}

	render(): React.ReactNode {
		const finishedStatusses = [
			FileUploadStatus.Done,
			FileUploadStatus.Failed,
			FileUploadStatus.Cancelled
		];

		const queueEmpty = !this.props.progress || this.props.progress.length === 0;
		const allItemsInQueueFinished = this.props.progress.every(item => finishedStatusses.indexOf(item.status) !== -1);

		return queueEmpty ? null : <div className="uploadProgress">
			<div className="header">
				{allItemsInQueueFinished && <button
					onClick={this.close}
					className="iconOnly">
					<IconClose/>
				</button>}
			</div>
			{this.props.progress.map(file => (
				<div key={file.file.name} className="file">
					<img src={file.objectUrl} className="thumb"/>
					<span className="title">{file.file.name}</span>
					<span className="status">{file.status}</span>
					{/* {file.status === "Failed" && <button>retry</button>} */}
				</div>
			))}
		</div>;
	}
}

export default UploadProgress;