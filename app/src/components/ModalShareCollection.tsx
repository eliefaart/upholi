import * as React from "react";
import Modal from "./Modal";
import AppStateContext from "../contexts/AppStateContext";
import { IconCopy } from "../components/Icons";
import { toast } from "react-toastify";
import Switch from "react-switch";
import { default as PhotoService, UpdateCollection } from "../services/PhotoService";
import ModalPropsBase from "../entities/ModalPropsBase";
import Collection from "../entities/Collection";

interface ModalShareCollectionProps extends ModalPropsBase {
	collection: Collection,
	onOptionsChanged: () => void
}

interface ModalShareCollectionState {
	shared: boolean,
	requirePassword: boolean,
	isChangingPassword: boolean,
	password: string
}


class ModalShareCollection extends React.Component<ModalShareCollectionProps, ModalShareCollectionState> {

	constructor(props: ModalShareCollectionProps) {
		super(props);

		this.state = {
			shared: this.props.collection.sharing.shared,
			requirePassword: this.props.collection.sharing.requirePassword,
			isChangingPassword: false,
			password: ""
		};
	}

	copyUrlToClipboard() {
		let publicUrlElement = document.getElementsByClassName("urlToCopy")[0] as HTMLInputElement;

		// Select text
		publicUrlElement.select();
		publicUrlElement.setSelectionRange(0, 99999);

		// Copy to clipboard
		document.execCommand("copy");

		// Deselect text
		publicUrlElement.blur();

		toast.info("URL copied to clipboard.");
	}

	onPasswordUpdated (password: string) {
		this.setState({
			isChangingPassword: false,
			password: password
		 }, () => {
			 this.updateSharingOptions();
		 });
	}

	updateSharingOptions() {
		const updateOptions: UpdateCollection = {
			title: null,
			albums: null,
			public: null,
			sharing: {
				shared: this.state.shared,
				requirePassword: this.state.requirePassword
			}
		};

		if (this.state.requirePassword && !!this.state.password) {
			updateOptions.sharing!.password = this.state.password;
		}

		PhotoService.updateCollection(this.props.collection.id, updateOptions)
			.then(_ => this.props.onOptionsChanged())
			.catch(console.error);
	}

	generateNewUrl() {
		PhotoService.rotateCollectionShareToken(this.props.collection.id)
			.then(_ => this.props.onOptionsChanged())
			.catch(console.error);
	}

	render() {
		let statusText = "This collection is private, only you can see it.";
		if (this.state.shared) {
			statusText = !this.state.requirePassword
				? "This collection is visible to anyone who has the link."
				: "This collection is visible to anyone who has the link, and knows the password.";
		}

		const shareUrl = document.location.origin + "/shared/collection/" + this.props.collection.sharing.token;

		return <Modal
			title="Sharing options"
			className={this.props.className + " modalShareCollection"}
			isOpen={this.props.isOpen}
			onRequestClose={this.props.onRequestClose}
			okButtonText="Done"
			onOkButtonClick={this.props.onRequestClose}
			>
				<p>
					{statusText}
				</p>
				<label className="switch">
					<span>Status</span>
					<Switch checked={this.state.shared}
						width={80}
						onColor="#53c253"
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
				{/* {this.state.shared && <label className="switch">
					<span>Require password</span>
					<Switch checked={this.state.requirePassword}
						width={80}
						onColor="#53c253"
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
				} */}

				{/* URL */}
				{this.state.shared && <div className="url">
					<div className="copyUrl">
						<input className="urlToCopy" type="text" value={shareUrl}
							// Prevent changes to the value of this input by resetting value in onchange event.
							// I cannot make it 'disabled', because then I cannot copy the text using JS
							onChange={(event) => event.target.value = shareUrl}/>
						<button className="iconOnly" onClick={() => this.copyUrlToClipboard()} title="Copy URL">
							<IconCopy/>
						</button>
					</div>
					<button onClick={() => this.generateNewUrl()}>
						Generate new URL
					</button>
				</div>}

			{this.state.isChangingPassword && <ModalSetPassword
				onOkButtonClick={(password: string) => this.onPasswordUpdated(password)}/>}
		</Modal>;
	}
}

interface ModalSetPasswordProps {
	onOkButtonClick: (password: string) => void
}


class ModalSetPassword extends React.Component<ModalSetPasswordProps> {
	constructor(props:ModalSetPasswordProps) {
		super(props);

		this.state = {};
	}

	onOkButtonClick() {
		const input = document.getElementById("password") as HTMLInputElement;
		if (input) {
			this.props.onOkButtonClick(input.value);
		}
	}

	render() {
		return <Modal
			title="Set password"
			className="modalSetPassword"
			isOpen={true}
			onRequestClose={() => {}}
			okButtonText="Save"
			onOkButtonClick={() => this.onOkButtonClick()}
			>
				<input id="password" type="password"/>
		</Modal>;
	}
}

ModalShareCollection.contextType = AppStateContext;
export default ModalShareCollection;