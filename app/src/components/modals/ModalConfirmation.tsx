import * as React from "react";
import Modal from "./Modal";
import ModalPropsBase from "../../models/ModalPropsBase";

interface ModalConfirmationProps extends ModalPropsBase {
	title?: string,
	onOkButtonClick: () => void,
	okButtonText?: string
	confirmationText: string
}

class ModalConfirmation extends React.Component<ModalConfirmationProps> {

	constructor(props: ModalConfirmationProps) {
		super(props);
	}

	componentDidMount() {
	}

	render() {
		return (
			<Modal
				title={this.props.title || "Confirmation"}
				isOpen={this.props.isOpen || false}
				onRequestClose={this.props.onRequestClose || null}
				onOkButtonClick={this.props.onOkButtonClick || null}
				okButtonText={this.props.okButtonText || "Ok"}
				>
					{this.props.confirmationText}
			</Modal>
		);
	}
}

export default ModalConfirmation;