import * as React from "react";
import * as ReactModal from "react-modal";
import { IconClose } from "../Icons";
import ModalPropsBase from "../../models/ModalPropsBase";

interface ModalProps extends ModalPropsBase {
	title: string,
	okButtonText?: string | null,
	headerActions?: JSX.Element,
	onOkButtonClick?: () => void
}

class Modal extends React.Component<ModalProps> {

	constructor(props: ModalProps) {
		super(props);
	}

	render() {
		return !this.props.isOpen ? null : (
			<ReactModal
				isOpen={this.props.isOpen}
				onRequestClose={this.props.onRequestClose}
				className={this.props.className + " modal"}
				overlayClassName="modalOverlay"
				shouldCloseOnOverlayClick={false}
			>
				<div className="modalHeader">
					<span className="title">{this.props.title}</span>
					{this.props.headerActions}
					<button className="iconOnly" onClick={() => this.props.onRequestClose()}>
						<IconClose/>
					</button>
				</div>
				<div className="modalBody">
					{this.props.children}
				</div>
				<div className="modalFooter">
					{this.props.okButtonText !== null && <button onClick={this.props.onOkButtonClick}>{this.props.okButtonText || "Ok"}</button>}
				</div>
			</ReactModal>
		);
	}
}

export default Modal;