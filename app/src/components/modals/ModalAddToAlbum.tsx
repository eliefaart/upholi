import * as React from "react";
import { FC } from "react";
import Modal from "./Modal";
import ModalPropsBase from "../../models/ModalPropsBase";
import { AlbumNew } from "../../models/Album";
import Albums from "../misc/Albums";

interface Props extends ModalPropsBase {
	onClickNewAlbum: (event: React.MouseEvent<HTMLElement, MouseEvent>) => void
	onClickExistingAlbum: (album: AlbumNew) => void
}

const ModalAddToAlbum: FC<Props> = (props) => {
	return <Modal
		title="Add to album"
		isOpen={props.isOpen}
		onRequestClose={props.onRequestClose}
		okButtonText={null}
		className={props.className + " modalAddToAlbum"}
		>
			<button onClick={props.onClickNewAlbum}>New album</button>
			<Albums onClick={props.onClickExistingAlbum}/>
	</Modal>;
};

export default ModalAddToAlbum;