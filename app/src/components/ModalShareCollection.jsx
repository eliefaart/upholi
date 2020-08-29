import React from "react";
import Modal from "./Modal.jsx";
import AppStateContext from "../contexts/AppStateContext.jsx";
import { IconCopy } from "../components/Icons.jsx";
import { toast } from "react-toastify";
import Switch from "react-switch";
import PhotoService from "../services/PhotoService.js";

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

	onPasswordUpdated (password) {
		// TODO: Send request to enable password + set password
		// TODO: Updating password should revoke access 
		// for all sessions that have been granted access to collection
		this.setState({ 
			isChangingPassword: false,
			password: password
		 }, () => {
			 this.updateSharingOptions();
		 });
	}

	updateSharingOptions() {
		const updateOptions = {
			sharing: {
				shared: this.state.shared,
				requirePassword: this.state.requirePassword
			}
		};

		if (this.state.requirePassword && !!this.state.password) {
			updateOptions.sharing.password = this.state.password;
		}

		PhotoService.updateCollection(this.props.collection.id, updateOptions)
			.catch(console.error);
	}

	generateNewUrl() {
		//PhotoService.updateCollection(this.props.collection.id, new )
	}

	render() {

		let statusText = "This collection is private, only you can see it.";
		if (this.state.shared) {
			statusText = !this.state.requirePassword
				? "This collection is visible to anyone who has the link."
				: "This collection is visible to anyone who has the link, and knows the password.";
		}

		return <Modal
			title="Sharing options"
			className={this.props.className + " modalShareCollection"}
			isOpen={this.props.isOpen}
			onRequestClose={this.props.onRequestClose}
			okButtonText="Done"
			>
				<p>
					{statusText}
				</p>
				<label className="switch">
					<span>Status</span>
					<Switch checked={this.state.shared}
						width={80}
						onColor="#d3e532"
						checkedIcon={<span className="checkedIcon">Shared</span>}
						uncheckedIcon={<span className="uncheckedIcon">Private</span>}
						onChange={(bShared) => {
							this.setState({ 
								shared: bShared 
							}, () => {
								this.updateSharingOptions();
							});
						}}/>
				</label>
				
				{/* Password */}
				{this.state.shared && <label className="switch">
					<span>Require password</span>
					<Switch checked={this.state.requirePassword}
						width={80}
						onColor="#d3e532"
						checkedIcon={<span className="checkedIcon">Yes</span>}
						uncheckedIcon={<span className="uncheckedIcon">No</span>}
						onChange={(bRequirePassword) => {
							this.setState({ 
								requirePassword: bRequirePassword,
								isChangingPassword: bRequirePassword
							}, () => {
								if (!bRequirePassword) {
									this.updateSharingOptions();
								}
							});
						}}/>
				</label>}
				{this.state.shared && this.state.requirePassword &&
					<button onClick={() => this.setState({ isChangingPassword: true })}>
						Change password
					</button>
				}

				{/* URL */}
				{this.state.shared && <div className="url">
					{/* <p>
						This collection can be shared using the link below.
					</p> */}
					<div className="copyUrl">
						<input className="urlToCopy" type="text" value={this.props.collection.id} 
							// Prevent changes to the value of this input by resetting value in onchange event.
							// I cannot make it 'disabled', because then I cannot copy the text using JS
							onChange={(event) => event.target.value = this.props.collection.id}/>
						<button className="iconOnly" onClick={() => this.copyUrlToClipboard()} title="Copy URL">
							<IconCopy/>
						</button>
					</div>
					<button onClick={() => this.generateNewUrl()}>
						Generate new URL
					</button>
				</div>}
					
			{this.state.isChangingPassword && <ModalSetPassword 
				onOkButtonClick={(password) => this.onPasswordUpdated(password)}/>}
		</Modal>;
	}
}

class ModalSetPassword extends React.Component {
	constructor(props) {
		super(props);

		this.state = {
		};
	}

	onOkButtonClick(event) {
		const input = document.getElementById("password");

		this.props.onOkButtonClick(input.value);
	}

	render() {
		return <Modal
			title="Set password"
			className={this.props.className + " modalSetPassword"}
			isOpen={true}
			onRequestClose={() => {}}
			okButtonText="Save"
			onOkButtonClick={(event) => this.onOkButtonClick(event)}
			>
				<input id="password" type="password"/>
		</Modal>;
	}
}

ModalShareCollection.contextType = AppStateContext;
export default ModalShareCollection;