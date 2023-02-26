import * as React from "react";
import { FC } from "react";
import Modal from "./Modal";
import appStateContext from "../../contexts/AppStateContext";
import { toast } from "react-toastify";
import ModalPropsBase from "../../models/ModalPropsBase";
import upholiService from "../../services/UpholiService";

interface Props extends ModalPropsBase {
  createWithPhotoIds?: string[];
}

const ModalCreateAlbum: FC<Props> = (props) => {
  const titleInput: React.RefObject<HTMLInputElement> = React.createRef();
  const context = React.useContext(appStateContext);

  const submitCreateAlbum = (): void => {
    if (titleInput.current) {
      const history = context.history;
      const title = titleInput.current.value;

      upholiService
        .createAlbum(title, props.createWithPhotoIds)
        .then((albumId) => {
          toast.info("Album '" + title + "' created.");
          history.push("/album/" + albumId);
        })
        .catch(console.error);
    }
  };

  return (
    <Modal
      title="Create album"
      className={props.className + " modalCreateAlbum"}
      isOpen={props.isOpen}
      onRequestClose={props.onRequestClose}
      onOkButtonClick={() => submitCreateAlbum()}
      okButtonText="Create"
    >
      <label>
        Title
        <input type="text" name="title" placeholder="Title" maxLength={40} ref={titleInput} />
      </label>
    </Modal>
  );
};

export default ModalCreateAlbum;
