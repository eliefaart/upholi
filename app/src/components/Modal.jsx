import React from 'react';
import { default as ReactModal } from 'react-modal';

class Modal extends React.Component {

	constructor(props) {
		super(props);
		
		ReactModal.setAppElement('#app');

		this.state = {
		};
	}
	
	componentDidMount() {
		
	}

	render() {
		return (
			<ReactModal
				isOpen={this.props.isOpen}
				onRequestClose={this.props.onRequestClose}
				className={this.props.className + " modal"}
				overlayClassName="overlay"
			>
				<div className="modal-header">
					<span className="title">{this.props.title}</span>
					<button className="button-close" onClick={() => this.props.onRequestClose()}>X</button>
				</div>
				<div className="modal-body">
					{this.props.children}
				</div>
				<div className="modal-footer">
					{this.props.okButtonText !== null && <button onClick={this.props.onOkButtonClick}>{this.props.okButtonText || "Ok"}</button>}
				</div>
			</ReactModal>
		);
	}
}

export default Modal;