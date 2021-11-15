import * as React from "react";
import { FC, useState } from "react";
import PhotoGallery from "../gallery/PhotoGallery";
import Content from "../layout/Content";
import appStateContext from "../../contexts/AppStateContext";
import ModalPhotoDetail from "../modals/ModalPhotoDetail";
import ModalConfirmation from "../modals/ModalConfirmation";
import UploadButton from "../misc/UploadButton";
import { IconDelete, IconUpload } from "../misc/Icons";
import { toast } from "react-toastify";
import UrlHelper from "../../helpers/UrlHelper";
import AddPhotosToAlbumButton from "../buttons/AddPhotosToAlbumButton";
import uploadHelper from "../../helpers/UploadHelper";
import upholiService from "../../services/UpholiService";
import { useTitle } from "../../hooks/useTitle";
import usePhotos from "../../hooks/usePhotos";
import usePhotoThumbnailSources from "../../hooks/usePhotoThumbnailSources";
import { PhotoMinimal } from "../../models/Photo";
import { elementIsInViewport } from "../../utils/dom";
import * as _ from "underscore";
import { PageProps } from "../../models/PageProps";

const queryStringParamNamePhotoId = "photoId";

const LibraryPage: FC<PageProps> = (props: PageProps) => {
	const context = React.useContext(appStateContext);
	const [photosThatHaveBeenInView, setPhotosThatHaveBeenInView] = useState<string[]>([]);
	const [photos, refreshPhotos] = usePhotos();
	const photoSources = usePhotoThumbnailSources(photos.filter(p => photosThatHaveBeenInView.some(id => id === p.id)));
	const [selectedPhotoIds, setSelectedPhotoIds] = useState<string[]>([]);
	const [openedPhotoId, setOpenedPhotoId] = useState<string>("");
	const [confirmDeletePhotosOpen, setConfirmDeletePhotosOpen] = useState<boolean>(false);

	const photosRef = React.useRef<PhotoMinimal[]>([]);
	photosRef.current = photos;

	useTitle("Library");
	React.useEffect(() => {
		props.setHeader({
			visible: true,
			headerActions: <React.Fragment>
				{selectedPhotoIds.length === 0 && <button
					className="iconOnly"
					onClick={() => {
						const element = document.getElementById("select-photos");
						if (element) {
							element.click();
						}
					}}
					title="Upload photos">
					<IconUpload/>
				</button>}
				<AddPhotosToAlbumButton
					selectedPhotoIds={selectedPhotoIds}
					onSelectionAddedToAlbum={() => setSelectedPhotoIds([])}/>
				{selectedPhotoIds.length > 0 && <button
					className="iconOnly"
					onClick={() => setConfirmDeletePhotosOpen(true)}
					title="Delete photos">
					<IconDelete/>
				</button>}
			</React.Fragment>
		});
	}, [selectedPhotoIds.length]);


	// Open photo, if indicated as such by query string
	const queryStringPhotoId = UrlHelper.getQueryStringParamValue(location.search, queryStringParamNamePhotoId);
	if (openedPhotoId !== queryStringPhotoId) {
		setOpenedPhotoId(queryStringPhotoId);
	}

	const loadVisiblePhotos = (): void => {
		// Find photo IDs currently in viewport
		const photoIdsInViewport = photosRef.current
			.filter(photo => {
				const photoElement = document.getElementById(photo.id);
				return photoElement && elementIsInViewport(photoElement);
			})
			.map(photo => photo.id);

		// Update state; merge photos currently in viewport with the ones that have been before.
		setPhotosThatHaveBeenInView((currentPhotoIds) => {
			const combined = currentPhotoIds.concat(photoIdsInViewport);
			const unique = [...new Set(combined)];

			return unique;
		});
	};

	const deleteSelectedPhotos = (): void => {
		upholiService.deletePhotos(selectedPhotoIds)
			.then(() => {
				setConfirmDeletePhotosOpen(false);
				setSelectedPhotoIds([]);
				refreshPhotos();
			});
	};

	const onPhotoClicked = (photoId: string): void => {
		if (photoId) {
			context.history.push(document.location.pathname + "?photoId=" + photoId);
		}
	};

	const onFilesDropped = (event: React.DragEvent<HTMLElement>): void => {
		event.preventDefault();
		if (!event.dataTransfer.files || event.dataTransfer.files.length === 0)
			return; // no files

		uploadFilesList(event.dataTransfer.files);
	};

	const uploadFilesList = (fileList: FileList): void => {
		const fnOnUploadFinished = () => {
			refreshPhotos();
			toast.info("Upload finished.");
		};

		uploadHelper.uploadPhotos(fileList).then(() => {
			fnOnUploadFinished();
		});
	};

	const onScrollThrottled = _.throttle(loadVisiblePhotos, 100);

	// Bind onscroll event handler
	React.useEffect(() => {
		const contentElement = document.getElementById("content");
		if (contentElement) {
			contentElement.addEventListener("scroll", onScrollThrottled);
		}
	}, []);

	// Load the initial batch of image thumbnails as soon as the first photos are available
	React.useEffect(() => {
		if (photosRef.current.length > 0) {
			loadVisiblePhotos();

			// Workaround; call that function again after some time.
			// This is because the Gallery component still seems to move and re-fit the photos a bit after its render function has completed,
			// and I don't see any event for when it has fully finished rendering.
			setTimeout(loadVisiblePhotos, 500);
		}
	}, [photos.length === 0]);

	const galleryPhotos = photos.map(photo => {
		return {
			id: photo.id,
			width: photo.width,
			height: photo.height,
			src: photoSources.find(p => p.photoId === photo.id)?.src ?? ""
		};
	});

	return <Content onDrop={onFilesDropped}>
		<PhotoGallery photos={galleryPhotos}
			onClick={onPhotoClicked}
			selectedItems={selectedPhotoIds}
			onPhotoSelectionChanged={setSelectedPhotoIds}
			/>

		{openedPhotoId && <ModalPhotoDetail
			isOpen={!!openedPhotoId}
			photoId={openedPhotoId}
			onRequestClose={() => context.history.push(document.location.pathname + "?" + UrlHelper.removeQueryStringParam(document.location.search, queryStringParamNamePhotoId))}
		/>}

		<ModalConfirmation
			title="Delete photos"
			isOpen={confirmDeletePhotosOpen}
			onRequestClose={() => setConfirmDeletePhotosOpen(false)}
			onOkButtonClick={() => deleteSelectedPhotos()}
			okButtonText="Delete"
			confirmationText={selectedPhotoIds.length + " photos will be deleted."}
			/>

		{/* Hidden upload button triggered by the button in action bar. This allows me to write simpler CSS to style the action buttons. */}
		<UploadButton className="hidden" onSubmit={uploadFilesList}/>
	</Content>;
};

export default LibraryPage;
