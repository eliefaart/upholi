import * as React from "react";
import { FC } from "react";
import Content from "../layout/Content";
import appStateContext from "../../contexts/AppStateContext";
import ModalConfirmation from "../modals/ModalConfirmation";
import UploadButton from "../misc/UploadButton";
import { IconRemove, IconImage, IconUpload, IconShare } from "../misc/Icons";
import { toast } from "react-toastify";
import ModalEditAlbum from "../modals/ModalEditAlbum";
import AddPhotosToAlbumButton from "../buttons/AddPhotosToAlbumButton";
import upholiService from "../../services/UpholiService";
import uploadHelper from "../../helpers/UploadHelper";
import ModalSharingOptions from "../modals/ModalSharingOptions";
import { SharingOptions } from "../../models/SharingOptions";
import AlbumView from "../AlbumView";
import { useTitle } from "../../hooks/useTitle";
import useAlbum from "../../hooks/useAlbum";
import useFoundAlbumShare from "../../hooks/useFoundAlbumShare";
import { PageProps } from "../../models/PageProps";

interface Props extends PageProps {
	// Note; this field represents the object set by react router
	match: {
		params: {
			albumId: string
		}
	};
}

const AlbumPage: FC<Props> = (props: Props) => {
	const albumId = props.match.params.albumId;
	const [album, refreshAlbum] = useAlbum(albumId);
	const [share, refreshShare] = useFoundAlbumShare(albumId);
	const [selectedPhotoIds, setSelectedPhotoIds] = React.useState<string[]>([]);
	const [editAlbumOpen, setEditAlbumOpen] = React.useState<boolean>(false);
	const [sharingOptionsOpen, setSharingOptionsOpen] = React.useState<boolean>(false);
	const [confirmDeleteAlbumOpen, setConfirmDeleteAlbumOpen] = React.useState<boolean>(false);
	const [confirmRemovePhotosOpen, setConfirmRemovePhotosOpen] = React.useState<boolean>(false);
	const context = React.useContext(appStateContext);

	const deleteAlbum = (): void => {
		const albumTitle = album?.title;

		upholiService.deleteAlbum(albumId)
			.then(() => {
				toast.info("Album '" + albumTitle + "' deleted.");
				context.history.push("/albums");
			})
			.catch(console.error);
	};

	const setSelectedPhotoAsAlbumCover = (): void => {
		const photoId = selectedPhotoIds[0];

		upholiService.updateAlbumCover(albumId, photoId)
			.then(() => {
				toast.info("Album cover updated.");
				setSelectedPhotoIds([]);
			})
			.catch(console.error);
	};

	const removeSelectedPhotosFromAlbum = (photoIds: string[]): void => {
		upholiService.removePhotosFromAlbum(albumId, photoIds)
			.then(() => {
				toast.info("Photos removed.");
				setConfirmRemovePhotosOpen(false);
				setSelectedPhotoIds([]);
				refreshAlbum();
			})
			.catch(console.error);
	};

	const onFilesDropped = (event: React.DragEvent<HTMLElement>): void => {
		event.preventDefault();
		if (!event.dataTransfer.files || event.dataTransfer.files.length === 0)
			return; // no files

		uploadFilesList(event.dataTransfer.files);
	};

	const uploadFilesList = (fileList: FileList): void => {
		const fnOnUploadFinished = () => {
			toast.info("Upload finished.");
			refreshAlbum();
		};

		uploadHelper.uploadPhotos(fileList).then((queue) => {
			if (album) {
				const photoIds = queue
					.map(file => file.uploadedPhotoId || "")
					.filter(id => !!id);
				upholiService.addPhotosToAlbum(album.id, photoIds)
					.finally(fnOnUploadFinished);
			}
		});
	};

	const updateSharingOptions = (options: SharingOptions): void => {
		const onShareUpdated = () => {
			refreshShare();
			toast.info("Sharing options updated");
		};

		if (options.shared) {
			upholiService.upsertAlbumShare(albumId, options.password)
				.then(onShareUpdated)
				.catch(console.error);
		}
		else {
			if (share) {
				upholiService.deleteShare(share.id)
					.then(onShareUpdated)
					.catch(console.error);
			}
		}
	};

	useTitle("Album - " + album?.title);
	React.useEffect(() => {
		props.setHeader({
			visible: true,
			headerActions: <>
				{selectedPhotoIds.length === 1 && <button className="icon-only" onClick={setSelectedPhotoAsAlbumCover} title="Set album cover">
					<IconImage />Set album cover
				</button>}
				<AddPhotosToAlbumButton
					selectedPhotoIds={selectedPhotoIds}
					onSelectionAddedToAlbum={() => setSelectedPhotoIds([])} />
				{selectedPhotoIds.length > 0 && <button className="icon-only" onClick={() => setConfirmRemovePhotosOpen(true)} title="Remove from album">
					<IconRemove />Remove from album
				</button>}
				{selectedPhotoIds.length === 0 && <button
					className="icon-only"
					onClick={() => {
						const selectPhotosElement = document.getElementById("select-photos");
						if (selectPhotosElement) {
							selectPhotosElement.click();
						}
					}}
					title="Upload photos">
					<IconUpload />Upload
				</button>}
				{selectedPhotoIds.length === 0 && <button
					className="icon-only"
					onClick={() => setSharingOptionsOpen(true)}
					title="Sharing options">
					<IconShare />Share
				</button>}
			</>,
			headerContextMenu: <>
				{<button onClick={() => setEditAlbumOpen(true)}>Edit album</button>}
				{<button onClick={() => setConfirmDeleteAlbumOpen(true)}>Delete album</button>}
			</>
		});
	}, [selectedPhotoIds.length]);

	if (!album) {
		return null;
	}
	else {
		return (
			<Content onDrop={(event) => onFilesDropped(event)}>
				<AlbumView
					album={album}
					selectedPhotos={selectedPhotoIds}
					onSelectionChanged={setSelectedPhotoIds}
				/>

				<ModalSharingOptions
					share={share}
					isOpen={sharingOptionsOpen}
					onOkButtonClick={() => null}
					onRequestClose={() => setSharingOptionsOpen(false)}
					onSharingOptionsUpdated={updateSharingOptions}
				/>

				<ModalEditAlbum
					isOpen={editAlbumOpen}
					onRequestClose={() => setEditAlbumOpen(false)}
					album={album} />

				<ModalConfirmation
					title="Delete album"
					isOpen={confirmDeleteAlbumOpen}
					onRequestClose={() => setConfirmDeleteAlbumOpen(false)}
					onOkButtonClick={() => deleteAlbum()}
					okButtonText="Delete"
					confirmationText={"Album '" + album.title + "' will be deleted."}
				/>

				<ModalConfirmation
					title="Remove photos"
					isOpen={confirmRemovePhotosOpen}
					onRequestClose={() => setConfirmRemovePhotosOpen(false)}
					onOkButtonClick={() => removeSelectedPhotosFromAlbum(selectedPhotoIds)}
					okButtonText="Remove"
					confirmationText={selectedPhotoIds.length + " photos will be removed from album '" + album.title + "'."}
				/>

				{/* Hidden upload button triggered by the button in action bar. This allos me to write simpler CSS to style the action buttons. */}
				<UploadButton className="hidden" onSubmit={uploadFilesList} />
			</Content>
		);
	}
};

export default AlbumPage;