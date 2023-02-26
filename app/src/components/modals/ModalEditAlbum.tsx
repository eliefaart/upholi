import * as React from "react";
import { FC } from "react";
import Modal from "./Modal";
import ModalPropsBase from "../../models/ModalPropsBase";
import { AlbumHydrated } from "../../models/Album";
import upholiService from "../../services/UpholiService";

interface Props extends ModalPropsBase {
  album: AlbumHydrated;
}

const ModalEditAlbum: FC<Props> = (props) => {
  const titleInput = React.createRef<HTMLInputElement>();
  const tagsInput = React.createRef<HTMLInputElement>();

  const saveChanges = (): void => {
    if (titleInput.current && tagsInput.current) {
      const title = titleInput.current.value;
      const tags = tagsInput.current.value
        .trim()
        .toLowerCase()
        .split(" ")
        .filter((tag) => !!tag);
      upholiService
        .updateAlbumTitleTags(props.album.id, title, tags)
        .then(() => props.onRequestClose())
        .catch(console.error);
    }
  };

  return (
    <Modal
      title={props.album.title}
      isOpen={props.isOpen}
      onRequestClose={props.onRequestClose}
      className={props.className + " modal-update-album"}
      okButtonText="Save"
      onOkButtonClick={saveChanges}
    >
      <label>
        Title
        <input type="text" placeholder="Title" ref={titleInput} defaultValue={props.album.title} />
      </label>

      <label>
        Tags
        <input type="text" placeholder="Tags" ref={tagsInput} defaultValue={props.album.tags.join(" ")} />
      </label>
    </Modal>
  );
};

export default ModalEditAlbum;
