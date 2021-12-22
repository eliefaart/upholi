import * as React from "react";
import { FC } from "react";
import * as ReactModal from "react-modal";
import { IconClose } from "../misc/Icons";
import ModalPropsBase from "../../models/ModalPropsBase";

interface Props extends ModalPropsBase {
	title: string,
	okButtonText?: string | null,
	headerActions?: JSX.Element,
	onOkButtonClick?: () => void,
	okButtonDisabled?: boolean
}

const Modal: FC<Props> = (props) => {
	if (!props.isOpen) {
		return null;
	}
	else {
		return <ReactModal
			isOpen={props.isOpen}
			onRequestClose={props.onRequestClose}
			className={props.className + " modal"}
			overlayClassName="modal-overlay"
			shouldCloseOnOverlayClick={false}
		>
			<div className="modal-header">
				<span className="title">{props.title}</span>
				{props.headerActions}
				<button className="with-icon" onClick={() => props.onRequestClose()}>
					<IconClose />
				</button>
			</div>
			<div className="modal-body">
				{props.children}
			</div>
			<div className="modal-footer">
				{props.okButtonText !== null && <button
					onClick={props.onOkButtonClick}
					disabled={props.okButtonDisabled}
				>
					{props.okButtonText || "Ok"}
				</button>}
			</div>
		</ReactModal>;
	}

};

export default Modal;