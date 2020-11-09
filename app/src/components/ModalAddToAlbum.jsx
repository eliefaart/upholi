import React from "react";
import Modal from "./Modal.jsx";
import AppStateContext from "../contexts/AppStateContext.ts";
import AllUserAlbums from "../components/AllUserAlbums.tsx";

class ModalAddToAlbum extends React.Component {

	constructor(props) {
		super(props);
	}

	submitCreateAlbum() {
	}

	render() {
		return <Modal
			title="Add to album"
			isOpen={this.props.isOpen}
			onRequestClose={this.props.onRequestClose}
			onOkButtonClick={null}
			okButtonText={null}
			className={this.props.className + " modalAddToAlbum"}
			>
				<button onClick={this.props.onClickNewAlbum}>New album</button>
				<AllUserAlbums onClick={this.props.onClickExistingAlbum}/>
		</Modal>;
	}
}

ModalAddToAlbum.contextType = AppStateContext;
export default ModalAddToAlbum;