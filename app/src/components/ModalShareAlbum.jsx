import React from 'react';
import Modal from './Modal.jsx';
import AppStateContext from '../contexts/AppStateContext.jsx';
import Switch from "react-switch";

class ModalShareAlbum extends React.Component {

	constructor(props) {
		super(props);
	}

	copyUrlToClipboard() {
		let publicUrlElement = document.getElementsByClassName("urlToCopy")[0];

		// Select text
		publicUrlElement.select();
		publicUrlElement.setSelectionRange(0, 99999);

		// Temporarily set disabled to false so we can copy the selected text
		document.execCommand("copy");
	}

	render() {
		return <Modal
			title="Share album"
			className={this.props.className + " modalShareAlbum"}
			isOpen={this.props.isOpen}
			onRequestClose={this.props.onRequestClose}
			onOkButtonClick={() => this.submitCreateAlbum()}
			okButtonText={null}
			>
				<label className="switch">
					<span>Public</span>
					<Switch checked={this.props.isPublic} onChange={(checked) => this.props.onPublicChanged(checked)}/>
				</label>

				{this.props.isPublic && 
					// This could be a component on its own if I need to reuse it.
					<div className="copyUrl">
						<input className="urlToCopy" type="text" value={this.props.publicUrl} 
							// Prevent changes to the value of this input by resetting value in onchange event.
							// I cannot make it 'disabled', because then I cannot copy the text using JS
							onChange={(event) => event.target.value = this.props.publicUrl}/>
						<button onClick={() => this.copyUrlToClipboard()}>Copy URL</button>
					</div>
				}
		</Modal>;
	}
}

ModalShareAlbum.contextType = AppStateContext;
export default ModalShareAlbum;