import * as React from "react";
import { FC } from "react";
import Modal from "./Modal";
import ModalPropsBase from "../../models/ModalPropsBase";
import { Album } from "../../models/Album";
import AlbumCollection from "../misc/Albums";
import useAlbums from "../../hooks/useAlbums";

interface Props extends ModalPropsBase {
  onClickNewAlbum: (event: React.MouseEvent<HTMLElement, MouseEvent>) => void;
  onClickExistingAlbum: (album: Album) => void;
}

const ModalAddToAlbum: FC<Props> = (props) => {
  const albums = useAlbums();

  return (
    <Modal
      title="Add to album"
      isOpen={props.isOpen}
      onRequestClose={props.onRequestClose}
      okButtonText={null}
      className={props.className + " modal-add-to-album"}
    >
      <button onClick={props.onClickNewAlbum}>New album</button>
      <AlbumCollection albums={albums} onClick={props.onClickExistingAlbum} />
    </Modal>
  );
};

export default ModalAddToAlbum;
