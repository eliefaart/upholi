import React from 'react';
import Modal from '../components/Modal.jsx';

class ConfirmationDialog extends React.Component {

	constructor(props) {
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

export default ConfirmationDialog;