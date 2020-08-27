import React from "react";
import Modal from "./Modal.jsx";
import AppStateContext from "../contexts/AppStateContext.jsx";
import Albums from "../components/Albums.jsx";

class ModalAddAlbumToCollection extends React.Component {

	constructor(props) {
		super(props);
	}

	submitCreateAlbum() {
	}

	render() {
		return <Modal
			title="Choose album to add to collection"
			isOpen={this.props.isOpen}
			onRequestClose={this.props.onRequestClose}
			onOkButtonClick={null}
			okButtonText={null}
			className={this.props.className + " modalAddAlbumToCollection"}
			>
				<Albums onClick={this.props.onAlbumSelected}/>
		</Modal>;
	}
}

ModalAddAlbumToCollection.contextType = AppStateContext;
export default ModalAddAlbumToCollection;