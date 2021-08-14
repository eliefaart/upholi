import * as React from "react";
import Modal from "./Modal";
import AppStateContext from "../../contexts/AppStateContext";
import { toast } from "react-toastify";
import ModalPropsBase from "../../models/ModalPropsBase";
import upholiService from "../../services/UpholiService";

interface ModalCreateAlbumProps extends ModalPropsBase {
	createWithPhotoIds?: string[]
}

class ModalCreateAlbum extends React.Component<ModalCreateAlbumProps> {
	titleInput: React.RefObject<HTMLInputElement>;

	constructor(props: ModalCreateAlbumProps) {
		super(props);

		this.titleInput = React.createRef();
	}

	submitCreateAlbum(): void {
		if (this.titleInput.current) {
			const history = this.context.history;
			const title = this.titleInput.current.value;

			upholiService.createAlbum(title) // , this.props.createWithPhotoIds ?? []
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
				<label>
					Title
					<input type="text" name="title" placeholder="Title" maxLength={40} ref={this.titleInput} />
				</label>
		</Modal>;
	}
}

ModalCreateAlbum.contextType = AppStateContext;
export default ModalCreateAlbum;