import * as React from "react";
import Modal from "./Modal";
import ModalPropsBase from "../models/ModalPropsBase";
import File from "../models/File";

interface ModalUploadProgressProps extends ModalPropsBase {
	files: File[]
}

class ModalUploadProgress extends React.Component<ModalUploadProgressProps> {

	constructor(props: ModalUploadProgressProps) {
		super(props);
	}

	render() {
		return (
			<Modal
				title="Upload progress"
				isOpen={this.props.isOpen}
				onRequestClose={this.props.onRequestClose}
				className={this.props.className + " modalUploadProgress"}
				okButtonText={null}
			>
				{this.props.files.map(file => (
					<div key={file.name} className="file">
						<img src={file.objectUrl} className="thumb"/>
						<span className="title">{file.name}</span>
						<span className="status">{file.status}</span>
						{/* {file.status === "Failed" && <button>retry</button>} */}
					</div>
				))}
			</Modal>
		);
	}
}

export default ModalUploadProgress;