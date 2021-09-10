import * as React from "react";
import Modal from "./Modal";
import Switch from "react-switch";
import { IconCopy } from "../Icons";
import { toast } from "react-toastify";
import ModalSetPassword from "./ModalSetPassword";

export interface SharingOptions {
	shared: boolean,
	password: string
}

interface Props {
	isOpen: boolean,
	onRequestClose?: () => void,
	onOkButtonClick: (password: string) => void,
	onSharingOptionsUpdated: (options: SharingOptions) => void
}

interface State {
	shared: boolean,
	password: string,
	isSettingPassword: boolean
}

export default class ModalSharingOptions extends React.Component<Props, State> {

	constructor(props:Props) {
		super(props);

		this.updateSharedState = this.updateSharedState.bind(this);
		this.updateSharingOptions = this.updateSharingOptions.bind(this);
		this.copyUrlToClipboard = this.copyUrlToClipboard.bind(this);
		this.onSetPasswordCancelled = this.onSetPasswordCancelled.bind(this);
		this.onSetPassword = this.onSetPassword.bind(this);

		this.state = {
			shared: false,
			password: "",
			isSettingPassword: false
		};
	}

	updateSharedState(shared: boolean): void {
		this.setState({
			shared,
			isSettingPassword: shared
		}, () => {
			this.updateSharingOptions();
		});
	}

	updateSharingOptions(): void {
		const options: SharingOptions = {
			shared: this.state.shared,
			password: this.state.password
		};

		this.props.onSharingOptionsUpdated(options);
	}

	onSetPasswordCancelled(): void {
		this.setState({
			isSettingPassword: false
		});
	}

	onSetPassword(password: string): void {
		this.setState({
			isSettingPassword: false,
			password: password
		}, () => {
			this.updateSharingOptions();
		});
	}

	copyUrlToClipboard(): void {
		const publicUrlElement = document.getElementsByClassName("urlToCopy")[0] as HTMLInputElement;

		// Select text
		publicUrlElement.select();
		publicUrlElement.setSelectionRange(0, 99999);

		// Copy to clipboard
		document.execCommand("copy");

		// Deselect text
		publicUrlElement.blur();

		toast.info("URL copied to clipboard.");
	}

	render(): React.ReactNode {
		const shareUrl = document.location.origin + "/s/" + "TODO";

		return <Modal
			title="Sharing options"
			className="modalSharingOptions"
			isOpen={this.props.isOpen}
			onRequestClose={() => {!!this.props.onRequestClose && this.props.onRequestClose();}}
			okButtonText="Save"
			onOkButtonClick={() => this.updateSharingOptions()}
			>
				<div>
					<label className="switch">
						<Switch checked={this.state.shared}
							width={40}
							height={15}
							handleDiameter={25}
							onColor="#20aced"
							offColor="#1c1c1c"
							checkedIcon={<span className="checkedIcon"/>}
							uncheckedIcon={<span className="uncheckedIcon"/>}
							onChange={this.updateSharedState}
							/>
						<span>Share via URL</span>
					</label>
				</div>

				{this.state.shared && <div className="url">
					<div className="copyUrl">
						<input className="urlToCopy" type="text" value={shareUrl}
							// Prevent changes to the value of this input by resetting value in onchange event.
							// I cannot make it 'disabled', because then I cannot copy the text using JS
							onChange={(event) => event.target.value = shareUrl}/>
						<button className="iconOnly" onClick={this.copyUrlToClipboard} title="Copy URL">
							<IconCopy/>
						</button>
					</div>
				</div>}

				{this.state.shared && <div className="flex flex-justify-content-right flex-grow">
					<button onClick={() => this.setState({ isSettingPassword: true })}>
						Reset password
					</button>
				</div>}

				{this.state.isSettingPassword && <ModalSetPassword
					onRequestClose={this.onSetPasswordCancelled}
					onOkButtonClick={this.onSetPassword}/>}
		</Modal>;
	}
}