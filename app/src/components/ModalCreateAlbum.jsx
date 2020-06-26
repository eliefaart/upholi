import React from 'react';
import $ from 'jquery';
import Modal from '../components/Modal.jsx';
import AppStateContext from '../contexts/AppStateContext.jsx';
import PhotoService from "../services/PhotoService.js";
import { toast } from 'react-toastify';

class ModalCreateAlbum extends React.Component {

	constructor(props) {
		super(props);
	}

	submitCreateAlbum() {
		let history = this.context.history;
		let form = $("#form-create-album");
		let title = form.find("[name=title]").val();

		PhotoService.createAlbum(title, this.props.createWithPhotos, (albumId) => {
			toast.info("Album '" + title + "' created.");
			
			if (history) {
				history.push("/album/" + albumId);
			}
		});
	}

	render() {
		return <Modal
			title="Create album"
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