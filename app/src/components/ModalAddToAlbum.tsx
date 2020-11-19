import * as React from "react";
import Modal from "./Modal";
import AppStateContext from "../contexts/AppStateContext";
import AllUserAlbums from "../components/AllUserAlbums";
import ModalPropsBase from "../entities/ModalPropsBase";
import AlbumInfo from "../entities/AlbumInfo";

interface ModalAddToAlbum extends ModalPropsBase {
	onClickNewAlbum: (event: React.MouseEvent<HTMLElement, MouseEvent>) => void
	onClickExistingAlbum: (album: AlbumInfo) => void
}

class ModalAddToAlbum extends React.Component<ModalAddToAlbum> {

	constructor(props: ModalAddToAlbum) {
		super(props);
	}

	submitCreateAlbum() {
	}

	render() {
		return <Modal
			title="Add to album"
			isOpen={this.props.isOpen}
			onRequestClose={this.props.onRequestClose}
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