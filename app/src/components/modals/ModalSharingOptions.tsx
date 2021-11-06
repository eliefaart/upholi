import * as React from "react";
import Modal from "./Modal";
import Switch from "react-switch";
import { SharingOptions } from "../../models/SharingOptions";
import { Share } from "../../models/Share";
import CopyUrl from "../misc/CopyUrl";

interface Props {
	share: Share | null,
	isOpen: boolean,
	onRequestClose?: () => void,
	onOkButtonClick: (password: string) => void,
	onSharingOptionsUpdated: (options: SharingOptions) => void
}

interface State {
	shared: boolean,
	password: string
}

export default class ModalSharingOptions extends React.Component<Props, State> {
	passwordInput: React.RefObject<HTMLInputElement>;

	constructor(props:Props) {
		super(props);

		this.passwordInput = React.createRef();

		this.updateSharedState = this.updateSharedState.bind(this);
		this.updateSharingOptions = this.updateSharingOptions.bind(this);

		this.state = {
			shared: !!this.props.share,
			password: ""
		};
	}

	updateSharedState(shared: boolean): void {
		this.setState({
			shared
		}, () => {
			this.updateSharingOptions();
		});
	}

	updateSharingOptions(): void {
		const options: SharingOptions = {
			shared: this.state.shared,
			password: this.passwordInput.current ? this.passwordInput.current.value : ""
		};

		this.props.onSharingOptionsUpdated(options);
	}

	render(): React.ReactNode {
		const shareUrl = document.location.origin + "/s/" + this.props.share?.id;

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
					Sharing URL
					<CopyUrl shareUrl={shareUrl}/>
				</div>}

				{this.state.shared && <label>
					Password
					<input type="text" defaultValue={this.props.share?.password} ref={this.passwordInput} placeholder=""/>
				</label>}
		</Modal>;
	}
}