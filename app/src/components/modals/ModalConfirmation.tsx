import * as React from "react";
import { FC } from "react";
import Modal from "./Modal";
import ModalPropsBase from "../../models/ModalPropsBase";

interface Props extends ModalPropsBase {
	title?: string,
	onOkButtonClick: () => void,
	okButtonText?: string
	confirmationText: string
}

const ModalConfirmation: FC<Props> = (props) => {
	return (
		<Modal
			title={props.title || "Confirmation"}
			isOpen={props.isOpen || false}
			onRequestClose={props.onRequestClose || null}
			onOkButtonClick={props.onOkButtonClick || null}
			okButtonText={props.okButtonText || "Ok"}
			>
				{props.confirmationText}
		</Modal>
	);
};

export default ModalConfirmation;