import React from "react";
import Modal from "./Modal.tsx";
import AppStateContext from "../contexts/AppStateContext.ts";
import AllUserAlbums from "../components/AllUserAlbums.tsx";

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
				<AllUserAlbums onClick={this.props.onAlbumSelected}/>
		</Modal>;
	}
}

ModalAddAlbumToCollection.contextType = AppStateContext;
export default ModalAddAlbumToCollection;