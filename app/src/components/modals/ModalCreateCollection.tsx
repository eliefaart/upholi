import * as React from "react";
import Modal from "./Modal";
import AppStateContext from "../../contexts/AppStateContext";
import ModalPropsBase from "../../models/ModalPropsBase";

interface ModalCreateCollectionProps extends ModalPropsBase {
	onOkButtonClick: (title: string) => void
}

class ModalCreateCollection extends React.Component<ModalCreateCollectionProps> {

	constructor(props: ModalCreateCollectionProps) {
		super(props);
	}

	onOkButtonClick(): void {
		const form = document.getElementById("form-create-collection");
		if (form) {
			const title = form.getElementsByTagName("input")[0].value;
			this.props.onOkButtonClick(title);
		}
	}

	render(): React.ReactNode {
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