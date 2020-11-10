import * as React from "react";
import * as ReactModal from "react-modal";
import { IconClose } from "../components/Icons";

interface ModalProps {
	isOpen: boolean,
	className?: string,
	title: string,
	okButtonText?: string | null,
	headerActions: JSX.Element,
	onRequestClose: (event: React.MouseEvent<Element, MouseEvent> | React.KeyboardEvent<Element>) => void,
	onOkButtonClick: (event: React.MouseEvent<HTMLButtonElement, MouseEvent>) => void
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
			>
				<div className="modalHeader">
					<span className="title">{this.props.title}</span>
					{this.props.headerActions}
					<button className="iconOnly" onClick={(event) => this.props.onRequestClose(event)}>
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