import React from "react";
import Modal from "./Modal.jsx";
import AppStateContext from "../contexts/AppStateContext.jsx";
import { IconCopy } from "../components/Icons.jsx";
import { toast } from "react-toastify";
import Switch from "react-switch";

class ModalShareCollection extends React.Component {

	constructor(props) {
		super(props);

		this.state = {
			shared: false,
			requirePassword: false,
			isChangingPassword: false
		};
	}

	copyUrlToClipboard() {
		let publicUrlElement = document.getElementsByClassName("urlToCopy")[0];

		// Select text
		publicUrlElement.select();
		publicUrlElement.setSelectionRange(0, 99999);

		// Copy to clipboard
		document.execCommand("copy");

		// Deselect text
		publicUrlElement.blur();

		toast.info("URL copied to clipboard.");
	}

	render() {
		return <Modal
			title="Sharing options"
			className={this.props.className + " modalShareCollection"}
			isOpen={this.props.isOpen}
			onRequestClose={this.props.onRequestClose}
			okButtonText={null}
			>
				<label className="switch">
					<Switch checked={this.state.shared}
						width={80}
						onColor="#d3e532"
						checkedIcon={<span className="checkedIcon">Shared</span>}
						uncheckedIcon={<span className="uncheckedIcon">Private</span>}
						onChange={(bShared) => {
							this.setState({ shared: bShared });
						}}/>
				</label>
				{this.state.shared && <div className="sharingOptions">
					<button>Generate new URL</button>
					<div className="copyUrl">
						<input className="urlToCopy" type="text" value={this.props.collection.id} 
							// Prevent changes to the value of this input by resetting value in onchange event.
							// I cannot make it 'disabled', because then I cannot copy the text using JS
							onChange={(event) => event.target.value = this.props.collection.id}/>
						<button className="iconOnly" onClick={() => this.copyUrlToClipboard()} title="Copy URL">
							<IconCopy/>
						</button>
					</div>
					<label className="switch">
						Require password
						<Switch checked={this.state.requirePassword}
							width={80}
							onColor="#d3e532"
							checkedIcon={<span className="checkedIcon">Yes</span>}
							uncheckedIcon={<span className="uncheckedIcon">No</span>}
							onChange={(bRequirePassword) => {
								this.setState({ 
									requirePassword: bRequirePassword,
									isChangingPassword: true
								 });
							}}/>
					</label>

					{/* This should go into modal, to make it flow better */}
					{this.state.requirePassword && <div className="passwordOptions">
						{this.state.isChangingPassword && <div>
							<input type="password"/>
							<button onClick={() => this.setState({
									isChangingPassword: false
								})}>
								Ok
							</button>
						</div>}
						{!this.state.isChangingPassword && <button 
							onClick={() => this.setState({
								isChangingPassword: true
							})}>
							Change password
						</button>}
					</div>}
						
					
				</div>}
		</Modal>;
	}
}

ModalShareCollection.contextType = AppStateContext;
export default ModalShareCollection;