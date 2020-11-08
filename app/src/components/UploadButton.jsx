import React from "react";
import { IconUpload } from "../components/Icons.tsx";

class UploadButton extends React.Component {

	constructor(props) {
		super(props);
	}

	onSubmitButtonClick(event) {
		if (!!this.props.onSubmit) {
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