import React from 'react';
import Modal from './Modal.jsx';
import AppStateContext from '../contexts/AppStateContext.jsx';

class ModalCopyUrl extends React.Component {

	constructor(props) {
		super(props);
	}

	copyUrlToClipboard() {
		let publicUrlElement = document.getElementsByClassName("urlToCopy")[0];

		// Select text
		publicUrlElement.select();
		publicUrlElement.setSelectionRange(0, 99999);

		// Copy to clipboard
		document.execCommand("copy");
	}

	render() {
		return <Modal
			title="Copy url"
			className={this.props.className + " modalCopyUrl"}
			isOpen={this.props.isOpen}
			onRequestClose={this.props.onRequestClose}
			okButtonText={null}
			>
				<div className="copyUrl">
					<input className="urlToCopy" type="text" value={this.props.url} 
						// Prevent changes to the value of this input by resetting value in onchange event.
						// I cannot make it 'disabled', because then I cannot copy the text using JS
						onChange={(event) => event.target.value = this.props.url}/>
					<button onClick={() => this.copyUrlToClipboard()}>Copy URL</button>
				</div>
		</Modal>;
	}
}

ModalCopyUrl.contextType = AppStateContext;
export default ModalCopyUrl;