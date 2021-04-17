import * as React from "react";
import Modal from "./Modal";
import AppStateContext from "../../contexts/AppStateContext";
import PhotoService from "../../services/PhotoService";
import { toast } from "react-toastify";
import ModalPropsBase from "../../models/ModalPropsBase";

interface ModalCreateAlbumProps extends ModalPropsBase {
	createWithPhotoIds?: string[]
}

class ModalCreateAlbum extends React.Component<ModalCreateAlbumProps> {

	constructor(props: ModalCreateAlbumProps) {
		super(props);
	}

	submitCreateAlbum(): void {
		const history = this.context.history;
		const form = document.getElementById("form-create-album");

		if (form) {
			const title = form.getElementsByTagName("input")[0].value;

			PhotoService.createAlbum(title, this.props.createWithPhotoIds ?? [])
				.then(albumId => {
					toast.info("Album '" + title + "' created.");

					if (history) {
						history.push("/album/" + albumId);
					}
				})
				.catch(console.error);
		}
	}

	render(): React.ReactNode {
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