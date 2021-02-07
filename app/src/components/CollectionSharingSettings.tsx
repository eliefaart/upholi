import * as React from "react";
import AppStateContext from "../contexts/AppStateContext";
import { IconCopy, IconRefresh } from "./Icons";
import { toast } from "react-toastify";
import Switch from "react-switch";
import { default as PhotoService, UpdateCollection } from "../services/PhotoService";
import Collection from "../models/Collection";
import Modal from "./modals/Modal";
import ModalConfirmation from "./modals/ModalConfirmation";

interface Props {
	collection: Collection,
	onOptionsChanged: () => void
}

interface State {
	requirePassword: boolean,
	password: string,
	isEnablingPasswordRequired: boolean,
	isSettingPassword: boolean,
	isConfirmingResetRefreshToken: boolean
}

class CollectionSharingSettings extends React.Component<Props, State> {

	constructor(props: Props) {
		super(props);

		this.onSetPasswordCancelled = this.onSetPasswordCancelled.bind(this);
		this.onSetPassword = this.onSetPassword.bind(this);

		this.state = {
			requirePassword: this.props.collection.sharing.requirePassword,
			password: "",
			isSettingPassword: false,
			isEnablingPasswordRequired: false,
			isConfirmingResetRefreshToken: false
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

	onSetPasswordCancelled() {
		this.setState({
			// If user was enabling password (first time setting password after checking 'require password'), but cancelled setting a password,
			// then also disable 'require password' again.
			requirePassword: this.state.isEnablingPasswordRequired ? false : this.state.requirePassword,
			isEnablingPasswordRequired: false,
			isSettingPassword: false
		 });
	}

	onSetPassword(password: string) {
		this.setState({
			isSettingPassword: false,
			isEnablingPasswordRequired: false,
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
				shared: true,
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
		const shareUrl = document.location.origin + "/s/" + this.props.collection.sharing.token;

		// Event handlers
		const fnOpenConfirmRefreshShareToken = () => this.setState({isConfirmingResetRefreshToken: true});
		const fnCloseConfirmRefreshShareToken = () => this.setState({isConfirmingResetRefreshToken: false});
		const fnOnConfirmRefreshTokenOk = () => {
			fnCloseConfirmRefreshShareToken();
			this.generateNewUrl();
		};

		return <React.Fragment>


			{/* URL */}
			<div className="url">
				<div className="copyUrl">
					<input className="urlToCopy" type="text" value={shareUrl}
						// Prevent changes to the value of this input by resetting value in onchange event.
						// I cannot make it 'disabled', because then I cannot copy the text using JS
						onChange={(event) => event.target.value = shareUrl}/>
					<button className="iconOnly" onClick={this.copyUrlToClipboard} title="Copy URL">
						<IconCopy/>
					</button>
				</div>
			</div>

			{/* Actions / Buttons */}
			<div className="flex">
				{/* Password */}
				<div className="flex">
					<label className="switch">
						<Switch checked={this.state.requirePassword}
							width={40}
							height={15}
							handleDiameter={25}
							onColor="#20aced"
							offColor="#1c1c1c"
							checkedIcon={<span className="checkedIcon"/>}
							uncheckedIcon={<span className="uncheckedIcon"/>}
							onChange={(bRequirePassword) => {
								this.setState({
									requirePassword: bRequirePassword,
									isEnablingPasswordRequired: bRequirePassword,
									isSettingPassword: bRequirePassword
								}, () => {
									if (!bRequirePassword) {
										this.updateSharingOptions();
									}
								});
							}}/>
						<span>Require password</span>
					</label>
				</div>

				<div className="flex flex-justify-content-right flex-grow">
					{this.state.requirePassword &&<button onClick={() => this.setState({ isSettingPassword: true })}>
						Reset password
					</button>}
					<button onClick={fnOpenConfirmRefreshShareToken}>
						Update URL
					</button>
				</div>
			</div>

			{this.state.isSettingPassword && <ModalSetPassword
				onRequestClose={this.onSetPasswordCancelled}
				onOkButtonClick={this.onSetPassword}/>}

			{this.state.isConfirmingResetRefreshToken && <ModalConfirmation
				title="Update URL"
				isOpen={true}
				onRequestClose={fnCloseConfirmRefreshShareToken}
				onOkButtonClick={fnOnConfirmRefreshTokenOk}
				okButtonText="Ok"
				confirmationText={"The old URL will no longer work."}
				/>}
		</React.Fragment>;
	}
}

interface ModalSetPasswordProps {
	onRequestClose?: () => void,
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
			onRequestClose={() => {!!this.props.onRequestClose && this.props.onRequestClose()}}
			okButtonText="Save"
			onOkButtonClick={() => this.onOkButtonClick()}
			>
				<input id="password" type="password"/>
		</Modal>;
	}
}

CollectionSharingSettings.contextType = AppStateContext;
export default CollectionSharingSettings;