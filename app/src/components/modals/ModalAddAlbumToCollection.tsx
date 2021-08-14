import * as React from "react";
import Modal from "./Modal";
import AppStateContext from "../../contexts/AppStateContext";
import AllUserAlbums from "../AllUserAlbums";
import ModalPropsBase from "../../models/ModalPropsBase";
import { AlbumNew } from "../../models/Album";

interface ModalAddAlbumToCollectionProps extends ModalPropsBase {
	onAlbumSelected: (album: AlbumNew) => void
}

class ModalAddAlbumToCollection extends React.Component<ModalAddAlbumToCollectionProps> {

	constructor(props: ModalAddAlbumToCollectionProps) {
		super(props);
	}

	render(): React.ReactNode {
		return <Modal
			title="Choose album to add to collection"
			isOpen={this.props.isOpen}
			onRequestClose={this.props.onRequestClose}
			okButtonText={null}
			className={this.props.className + " modalAddAlbumToCollection"}
			>
				<AllUserAlbums onClick={this.props.onAlbumSelected}/>
		</Modal>;
	}
}

ModalAddAlbumToCollection.contextType = AppStateContext;
export default ModalAddAlbumToCollection;