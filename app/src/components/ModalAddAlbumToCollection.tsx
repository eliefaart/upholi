import * as React from "react";
import Modal from "./Modal";
import AppStateContext from "../contexts/AppStateContext";
import AllUserAlbums from "../components/AllUserAlbums";
import ModalPropsBase from "../entities/ModalPropsBase";
import AlbumInfo from "../entities/AlbumInfo";

interface ModalAddAlbumToCollectionProps extends ModalPropsBase {
	onAlbumSelected: (album: AlbumInfo) => void
}

class ModalAddAlbumToCollection extends React.Component<ModalAddAlbumToCollectionProps> {

	constructor(props: ModalAddAlbumToCollectionProps) {
		super(props);
	}

	render() {
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