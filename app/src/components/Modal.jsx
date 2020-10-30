import React from "react";
import { default as ReactModal } from "react-modal";
import { IconClose } from "../components/Icons.jsx";

class Modal extends React.Component {

	constructor(props) {
		super(props);
	}

	render() {
		return !this.props.isOpen ? null : (
			<ReactModal
				isOpen={this.props.isOpen}
				onRequestClose={this.props.onRequestClose}
				className={this.props.className + " modal"}
				overlayClassName="overlay"
			>
				<div className="modalHeader">
					<span className="title">{this.props.title}</span>
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