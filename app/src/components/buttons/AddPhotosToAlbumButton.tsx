import * as React from "react";
import { FC } from "react";
import { toast } from "react-toastify";
import { Album } from "../../models/Album";
import upholiService from "../../services/UpholiService";
import Button from "../misc/Button";
import { IconAddToAlbum } from "../misc/Icons";
import ModalAddToAlbum from "../modals/ModalAddToAlbum";
import ModalCreateAlbum from "../modals/ModalCreateAlbum";

interface Props {
	selectedPhotoIds: string[];
	onSelectionAddedToAlbum?: () => void;
}

const AddPhotosToAlbumButton: FC<Props> = (props) => {
	const [selectAlbumModelOpen, setSelectAlbumModelOpen] = React.useState(false);
	const [createAlbumModelOpen, setCreateAlbumModelOpen] = React.useState(false);

	const addSelectedPhotosToAlbum = (album: Album): void => {
		upholiService.addPhotosToAlbum(album.id, props.selectedPhotoIds)
			.then(() => {
				setSelectAlbumModelOpen(false);

				toast.info("Photos added to album.");

				if (props.onSelectionAddedToAlbum) {
					props.onSelectionAddedToAlbum();
				}
			})
			.catch(console.error);
	};

	const openSelectAlbumModal = (): void => {
		setSelectAlbumModelOpen(true);
	};

	const openCreateAlbumModal = (): void => {
		setCreateAlbumModelOpen(true);
	};

	if (props.selectedPhotoIds.length === 0) {
		return null;
	}
	else {
		return <>
			<Button onClick={openSelectAlbumModal}
				label="Add to album"
				icon={<IconAddToAlbum />} />

			<ModalAddToAlbum
				isOpen={selectAlbumModelOpen}
				onRequestClose={() => setSelectAlbumModelOpen(false)}
				onClickNewAlbum={openCreateAlbumModal}
				onClickExistingAlbum={addSelectedPhotosToAlbum}
			/>

			<ModalCreateAlbum
				isOpen={createAlbumModelOpen}
				onRequestClose={() => setCreateAlbumModelOpen(false)}
				createWithPhotoIds={props.selectedPhotoIds}
			/>
		</>;
	}
};

export default AddPhotosToAlbumButton;