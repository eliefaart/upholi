import * as React from "react";
import Modal from "./Modal";

interface Props {
	onRequestClose?: () => void,
	onOkButtonClick: (password: string) => void
}

export default class ModalSetPassword extends React.Component<Props> {
	constructor(props:Props) {
		super(props);

		this.state = {};
	}

	onOkButtonClick(): void {
		const input = document.getElementById("password") as HTMLInputElement;
		if (input) {
			this.props.onOkButtonClick(input.value);
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
				<input id="password" type="password"/>
		</Modal>;
	}
}