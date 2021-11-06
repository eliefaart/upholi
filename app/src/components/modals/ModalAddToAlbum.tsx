import * as React from "react";
import Modal from "./Modal";
import appStateContext from "../../contexts/AppStateContext";
import ModalPropsBase from "../../models/ModalPropsBase";
import { AlbumNew } from "../../models/Album";
import Albums from "../misc/Albums";

interface ModalAddToAlbumProps extends ModalPropsBase {
	onClickNewAlbum: (event: React.MouseEvent<HTMLElement, MouseEvent>) => void
	onClickExistingAlbum: (album: AlbumNew) => void
}

class ModalAddToAlbum extends React.Component<ModalAddToAlbumProps> {

	constructor(props: ModalAddToAlbumProps) {
		super(props);
	}

	render(): React.ReactNode {
		return <Modal
			title="Add to album"
			isOpen={this.props.isOpen}
			onRequestClose={this.props.onRequestClose}
			okButtonText={null}
			className={this.props.className + " modalAddToAlbum"}
			>
				<button onClick={this.props.onClickNewAlbum}>New album</button>
				<Albums onClick={this.props.onClickExistingAlbum}/>
		</Modal>;
	}
}

ModalAddToAlbum.contextType = appStateContext;
export default ModalAddToAlbum;