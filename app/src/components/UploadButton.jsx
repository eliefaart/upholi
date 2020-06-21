import React from 'react';

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
		return <form id="form-select-photos">
			<label htmlFor="select-photos" className="asButton">Upload</label>
			<input id="select-photos" type="file" name="photos" accept=".jpg,.jpeg" onChange={(event) => this.onSubmitButtonClick(event)} multiple/>
		</form>;
	}
}

export default UploadButton;