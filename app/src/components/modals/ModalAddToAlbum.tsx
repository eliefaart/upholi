import * as React from "react";
import { FC } from "react";
import Modal from "./Modal";
import ModalPropsBase from "../../models/ModalPropsBase";
import { AlbumPlain } from "../../models/Album";
import Albums from "../misc/Albums";
import useAlbums from "../../hooks/useAlbums";

interface Props extends ModalPropsBase {
	onClickNewAlbum: (event: React.MouseEvent<HTMLElement, MouseEvent>) => void
	onClickExistingAlbum: (album: AlbumPlain) => void
}

const ModalAddToAlbum: FC<Props> = (props) => {
	const [albums] = useAlbums();

	return <Modal
		title="Add to album"
		isOpen={props.isOpen}
		onRequestClose={props.onRequestClose}
		okButtonText={null}
		className={props.className + " modal-add-to-album"}
	>
		<button onClick={props.onClickNewAlbum}>New album</button>
		<Albums albums={albums} onClick={props.onClickExistingAlbum} />
	</Modal>;
};

export default ModalAddToAlbum;