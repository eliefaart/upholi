import React from "react";
import Modal from "../components/Modal.jsx";
import AppStateContext from "../contexts/AppStateContext.ts";
import PhotoService from "../services/PhotoService.ts";
import { toast } from "react-toastify";

class ModalCreateAlbum extends React.Component {

	constructor(props) {
		super(props);
	}

	submitCreateAlbum() {
		const history = this.context.history;
		const form = document.getElementById("form-create-album");
		const title = form.getElementsByTagName("input")[0].value;

		PhotoService.createAlbum(title, this.props.createWithPhotos)
			.then(albumId => {
				toast.info("Album '" + title + "' created.");

				if (history) {
					history.push("/album/" + albumId);
				}
			})
			.catch(console.error);
	}

	render() {
		return <Modal
			title="Create album"
			className={this.props.className + " modalCreateAlbum"}
			isOpen={this.props.isOpen}
			onRequestClose={this.props.onRequestClose}
			onOkButtonClick={() => this.submitCreateAlbum()}
			okButtonText="Create"
			>
				<form id="form-create-album">
					<input name="title" placeholder="Title" maxLength={40}/>
				</form>
		</Modal>;
	}
}

ModalCreateAlbum.contextType = AppStateContext;
export default ModalCreateAlbum;