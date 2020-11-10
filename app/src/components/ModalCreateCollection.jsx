import React from "react";
import Modal from "../components/Modal.tsx";
import AppStateContext from "../contexts/AppStateContext.ts";

class ModalCreateCollection extends React.Component {

	constructor(props) {
		super(props);
	}

	onOkButtonClick() {
		const form = document.getElementById("form-create-collection");
		const title = form.getElementsByTagName("input")[0].value;

		this.props.onOkButtonClick(title);
	}

	render() {
		return <Modal
			title="Create collection"
			className={this.props.className + " modalCreateCollection"}
			isOpen={this.props.isOpen}
			onRequestClose={this.props.onRequestClose}
			onOkButtonClick={() => this.onOkButtonClick()}
			okButtonText="Create"
			>
				<form id="form-create-collection">
					<input name="title" placeholder="Title" maxLength={40}/>
				</form>
		</Modal>;
	}
}

ModalCreateCollection.contextType = AppStateContext;
export default ModalCreateCollection;