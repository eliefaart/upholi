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
	isChangingPassword: boolean,
	password: string,
	isConfirmingRefreshToken: boolean
}

class CollectionSharingSettings extends React.Component<Props, State> {

	constructor(props: Props) {
		super(props);

		this.state = {
			requirePassword: this.props.collection.sharing.requirePassword,
			isChangingPassword: false,
			password: "",
			isConfirmingRefreshToken: false
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

	onPasswordUpdated(password: string) {
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
		const fnOpenConfirmRefreshShareToken = () => this.setState({isConfirmingRefreshToken: true});
		const fnCloseConfirmRefreshShareToken = () => this.setState({isConfirmingRefreshToken: false});
		const fnOnConfirmRefreshTokenOk = () => {
			fnCloseConfirmRefreshShareToken();
			this.generateNewUrl();
		};

		return <React.Fragment>
			{/* Password */}
			{/* <label className="switch">
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
			</label>
			{this.state.requirePassword &&
				<button onClick={() => this.setState({ isChangingPassword: true })}>
					Change password
				</button>
			} */}

			{/* URL */}
			<div className="url">
				<div className="copyUrl">
					<input className="urlToCopy" type="text" value={shareUrl}
						// Prevent changes to the value of this input by resetting value in onchange event.
						// I cannot make it 'disabled', because then I cannot copy the text using JS
						onChange={(event) => event.target.value = shareUrl}/>
					<button className="iconOnly" onClick={fnOpenConfirmRefreshShareToken} title="Update URL">
						<IconRefresh/>
					</button>
					<button className="iconOnly" onClick={this.copyUrlToClipboard} title="Copy URL">
						<IconCopy/>
					</button>
				</div>
			</div>

			{this.state.isChangingPassword && <ModalSetPassword
				onOkButtonClick={(password: string) => this.onPasswordUpdated(password)}/>}

			{this.state.isConfirmingRefreshToken && <ModalConfirmation
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

CollectionSharingSettings.contextType = AppStateContext;
export default CollectionSharingSettings;