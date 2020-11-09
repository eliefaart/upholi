import * as React from "react";
import { IconUpload } from "./Icons";

interface UploadButtonProps {
	className: string,
	onSubmit: (fileList: FileList) => void
}

class UploadButton extends React.Component<UploadButtonProps> {

	constructor(props: UploadButtonProps) {
		super(props);
	}

	onSubmitButtonClick(event: React.ChangeEvent<HTMLInputElement>) {
		if (this.props.onSubmit && event.target.files) {
			this.props.onSubmit(event.target.files);
		}
	}

	render() {
		return <form id="form-select-photos" className={this.props.className}>
			<label htmlFor="select-photos" className="asButton"><IconUpload/></label>
			<input id="select-photos" type="file" name="photos" accept=".jpg,.jpeg" onChange={(event) => this.onSubmitButtonClick(event)} multiple/>
		</form>;
	}
}

export default UploadButton;