import * as React from "react";
import Modal from "./Modal";

interface Props {
	onRequestClose?: () => void,
	onOkButtonClick: (password: string) => void
}

export default class ModalSetPassword extends React.Component<Props> {
	passwordInput: React.RefObject<HTMLInputElement>;

	constructor(props:Props) {
		super(props);

		this.passwordInput = React.createRef();

		this.state = {};
	}

	onOkButtonClick(): void {
		if (this.passwordInput.current) {
			this.props.onOkButtonClick(this.passwordInput.current.value);
		}
	}

	render(): React.ReactNode {
		return <Modal
			title="Set password"
			className="modalSetPassword"
			isOpen={true}
			onRequestClose={() => {!!this.props.onRequestClose && this.props.onRequestClose();}}
			okButtonText="Save"
			onOkButtonClick={() => this.onOkButtonClick()}
			>
				<label>
					Password
					<input type="password" ref={this.passwordInput} placeholder="password"/>
				</label>
		</Modal>;
	}
}