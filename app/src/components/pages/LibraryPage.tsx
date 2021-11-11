import * as React from "react";
import { FC, useState } from "react";
import PhotoGallery from "../misc/PhotoGallery";
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
import { setHeader } from "../../hooks/useHeader";
import usePhotos from "../../hooks/usePhotos";
import usePhotoThumbnailSources from "../../hooks/usePhotoThumbnailSources";

const queryStringParamNamePhotoId = "photoId";

const LibraryPage: FC = () => {
	const context = React.useContext(appStateContext);
	const [photos, refreshPhotos] = usePhotos();
	const photoSources = usePhotoThumbnailSources(photos);
	const [selectedPhotoIds, setSelectedPhotoIds] = useState<string[]>([]);
	const [openedPhotoId, setOpenedPhotoId] = useState<string>("");
	const [confirmDeletePhotosOpen, setConfirmDeletePhotosOpen] = useState<boolean>(false);

	// Open photo, if indicated as such by query string
	const queryStringPhotoId = UrlHelper.getQueryStringParamValue(location.search, queryStringParamNamePhotoId);
	if (openedPhotoId !== queryStringPhotoId) {
		setOpenedPhotoId(queryStringPhotoId);
	}

	// const photosWithoutSource = photos.filter(p => photoSources.some(ps => ps.photoId === p.id));
	// if (photosWithoutSource.length > 0) {
	// 	const newPhotoSources = photosWithoutSource.map<PhotoSources>(p => {
	// 		return {
	// 			photoId: p.id,
	// 			src: ""
	// 		};
	// 	});

	// 	for (const photoSource of newPhotoSources) {
	// 		upholiService.getPhotoThumbnailImageSrc(photoSource.photoId)
	// 			.then(src => {

	// 				setPhotoSources(photoSources);
	// 			});
	// 	}

	// 	setPhotoSources(photoSources.concat(newPhotoSources));
	// }






	// const loadVisiblePhotos = (): void => {
	// 	console.log("loadVisiblePhotos", photos);
	// 	// Function that checks if given element is at least partially visible within viewport
	// 	const fnElementIsInViewport = (element: HTMLElement) => {
	// 		if (!element)  {
	// 			return false;
	// 		}
	// 		else {
	// 			const myElementHeight = element.offsetHeight;
	// 			const myElementWidth = element.offsetWidth;
	// 			const bounding = element.getBoundingClientRect();

	// 			return bounding.top >= -myElementHeight
	// 				&& bounding.left >= -myElementWidth
	// 				&& bounding.right <= (window.innerWidth || document.documentElement.clientWidth) + myElementWidth
	// 				&& bounding.bottom <= (window.innerHeight || document.documentElement.clientHeight) + myElementHeight;
	// 		}
	// 	};

	// 	for (const photo of photos) {
	// 		const photoHasBeenLoaded = photoSources.some(ps => ps.photoId === photo.id);
	// 		console.log(photo.id, photoHasBeenLoaded, photoSources);
	// 		if (!photoHasBeenLoaded) {
	// 			const photoElement = document.getElementById(photo.id);

	// 			if (photoElement && fnElementIsInViewport(photoElement)) {
	// 				upholiService.getPhotoThumbnailImageSrc(photo.id)
	// 					.then(src => {
	// 						photoSources.push({
	// 							photoId: photo.id,
	// 							src
	// 						});
	// 						setPhotoSources(photoSources);

	// 						// this.setState(previousState => {
	// 						// 	const photo = previousState.photos.find(p => p.id === photo.id);
	// 						// 	if (photo) {
	// 						// 		photo.src = src;
	// 						// 	}

	// 						// 	return {
	// 						// 		photos: previousState.photos
	// 						// 	};
	// 						// });
	// 					});
	// 			}
	// 		}
	// 	}
	// };

	const waitingForPhotoCheck = false;
	let photoCheckQueued = false;
	const onScroll = () => {
		console.log("onScroll");
		// No need to check check photo visibility every time the scroll event fires,
		// because it may fire many times per second. Limiting to every 50ms is more than enough.
		const msTimeBetweenChecks = 50;

		if (!waitingForPhotoCheck) {
			photoCheckQueued = true;
			setTimeout(() => {
				//loadVisiblePhotos();
				photoCheckQueued = false;
			}, msTimeBetweenChecks);
		}
	};

	const deleteSelectedPhotos = (): void => {
		upholiService.deletePhotos(selectedPhotoIds)
			.then(() => {
				setConfirmDeletePhotosOpen(false);
				setSelectedPhotoIds([]);
				refreshPhotos();
			});
	};

	// const onPhotoSelectedChange = (photoId: string, selected: boolean): void => {
	// 	const selectedPhotos = selectedPhotoIds;

	// 	if (selected) {
	// 		selectedPhotos.push(photoId);
	// 	} else {
	// 		const index = selectedPhotos.indexOf(photoId);
	// 		if (index > -1) {
	// 			selectedPhotos.splice(index, 1);
	// 		}
	// 	}

	// 	setSelectedPhotoIds(selectedPhotos);
	// };

	const onPhotoClicked = (index: number): void => {
		const photo = photos[index];
		context.history.push(document.location.pathname + "?photoId=" + photo.id);
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

	React.useEffect(() => {
		const contentElement = document.getElementById("content");
		if (contentElement) {
			contentElement.addEventListener("scroll", onScroll);
		}
	}, []);

	useTitle("Library");
	//React.useEffect(() => {
		setHeader({
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
	//}, [selectedPhotoIds.length]);


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
				onClick={(_, target) => onPhotoClicked(target.index)}
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

			{/* Hidden upload button triggered by the button in action bar. This allos me to write simpler CSS to style the action buttons. */}
			<UploadButton className="hidden" onSubmit={uploadFilesList}/>
		</Content>;
};

export default LibraryPage;

// class LibraryPage extends PageBaseComponent<LibraryPageState> {

// 	photos: PhotoMinimal[];
// 	waitingForPhotoCheck: boolean;
// 	photoCheckQueued: boolean;
// 	onScroll: () => void;

// 	constructor(props: PageBaseComponentProps) {
// 		super(props);

// 		// Contains all user's photos, but this is not the viewmodel of the Gallery
// 		this.photos = [];

// 		this.waitingForPhotoCheck = false;
// 		this.photoCheckQueued = false;
// 		this.onScroll = () => {
// 			// No need to check check photo visibility every time the scroll event fires,
// 			// because it may fire many times per second. Limiting to every 50ms is more than enough.
// 			const msTimeBetweenChecks = 50;

// 			if (!this.waitingForPhotoCheck) {
// 				this.photoCheckQueued = true;
// 				setTimeout(() => {
// 					this.loadVisiblePhotos();
// 					this.photoCheckQueued = false;
// 				}, msTimeBetweenChecks);
// 			}
// 		};

// 		this.resetSelection = this.resetSelection.bind(this);
// 		this.refreshPhotos = this.refreshPhotos.bind(this);

// 		this.state = {
// 			photos: [],
// 			selectedPhotoIds: [],
// 			openedPhotoId: null,
// 			confirmDeletePhotosOpen: false,
// 			albums: []
// 		};
// 	}

// 	componentDidUpdate(prevProps: PageBaseComponentProps, prevState: LibraryPageState): void {
// 		this.context.headerActions = this.getHeaderActions();
// 		this.context.headerContextMenu = this.getHeaderContextMenu();

// 		// Load the initial batch of photo thumbnails once all photo data has been fetched
// 		const anyPhotoLoaded = this.state.photos.some(p => !!p.src);
// 		if (!anyPhotoLoaded) {
// 			this.loadVisiblePhotos();

// 			// Workaround:
// 			// Call that function again after some time.
// 			// This is because the Gallery component still seems to move and re-fit the photos a bit after its render function has completed,
// 			// and I don't see any event for when it has fully finished rendering.
// 			setTimeout(() => this.loadVisiblePhotos(), 500);
// 		}

// 		// Remove onscroll event handler from body once all photos have been loaded
// 		if (this.state.photos.length > 0) {
// 			const nPhotosNotYetLoaded = this.state.photos.filter(photo => photo.src === "").length;
// 			if (nPhotosNotYetLoaded === 0) {
// 				const contentElement = document.getElementById("content");
// 				if (contentElement) {
// 					contentElement.removeEventListener("scroll", this.onScroll);
// 				}
// 			}
// 		}

// 		// Open photo, if indicated as such by query string
// 		const queryStringPhotoId = UrlHelper.getQueryStringParamValue(location.search, queryStringParamNamePhotoId);
// 		if (openedPhotoId !== queryStringPhotoId) {
// 			this.setState({
// 				openedPhotoId: queryStringPhotoId
// 			});
// 		}

// 		super.componentDidUpdate(prevProps, prevState);
// 	}

// 	/**
// 	 * Load all photo thumbnails that are visible in the viewport
// 	 */
// 	loadVisiblePhotos(): void {
// 		// Function that checks if given element is at least partially visible within viewport
// 		const fnElementIsInViewport = (element: HTMLElement) => {
// 			if (!element)  {
// 				return false;
// 			}
// 			else {
// 				const myElementHeight = element.offsetHeight;
// 				const myElementWidth = element.offsetWidth;
// 				const bounding = element.getBoundingClientRect();

// 				return bounding.top >= -myElementHeight
// 					&& bounding.left >= -myElementWidth
// 					&& bounding.right <= (window.innerWidth || document.documentElement.clientWidth) + myElementWidth
// 					&& bounding.bottom <= (window.innerHeight || document.documentElement.clientHeight) + myElementHeight;
// 			}
// 		};

// 		const statePhotos = this.state.photos;

// 		for (const statePhoto of statePhotos) {
// 			const photoHasBeenLoaded = statePhoto.src !== "";
// 			if (!photoHasBeenLoaded) {
// 				const photoInfo = this.photos.find(p => p.id === statePhoto.id);
// 				const photoElement = document.getElementById(statePhoto.id);

// 				if (photoElement && photoInfo && fnElementIsInViewport(photoElement)) {
// 					upholiService.getPhotoThumbnailImageSrc(photoInfo.id)
// 						.then(src => {
// 							this.setState(previousState => {
// 								const photo = previousState.photos.find(p => p.id === statePhoto.id);
// 								if (photo) {
// 									photo.src = src;
// 								}

// 								return {
// 									photos: previousState.photos
// 								};
// 							});
// 						});
// 				}
// 			}
// 		}
// 	}

// 	resetSelection(): void {
// 		this.setState({
// 			selectedPhotoIds: []
// 		});
// 	}

// 	onClickDeletePhotos(): void {
// 		this.setState({
// 			confirmDeletePhotosOpen: true
// 		});
// 	}



// 	render(): React.ReactNode {
// 		return (
// 			<Content onDrop={(event) => this.onFilesDropped(event)}>
// 				<PhotoGallery photos={this.state.photos} onClick={(_, target) => this.onPhotoClicked(target.index)} selectedItems={selectedPhotoIds} onPhotoSelectedChange={(photoId, selected) => this.onPhotoSelectedChange(photoId, selected)} />

// 				{openedPhotoId && <ModalPhotoDetail
// 					isOpen={!!openedPhotoId}
// 					photoId={openedPhotoId}
// 					onRequestClose={() => this.context.history.push(document.location.pathname + "?" + UrlHelper.removeQueryStringParam(document.location.search, queryStringParamNamePhotoId))}
// 				/>}

// 				<ModalConfirmation
// 					title="Delete photos"
// 					isOpen={confirmDeletePhotosOpen}
// 					onRequestClose={() => this.setState({confirmDeletePhotosOpen: false})}
// 					onOkButtonClick={() => this.deleteSelectedPhotos()}
// 					okButtonText="Delete"
// 					confirmationText={selectedPhotoIds.length + " photos will be deleted."}
// 					/>

// 				{/* Hidden upload button triggered by the button in action bar. This allos me to write simpler CSS to style the action buttons. */}
// 				<UploadButton className="hidden" onSubmit={(files) => this.uploadFilesList(files)}/>
// 			</Content>
// 		);
// 	}
// }

// LibraryPage.contextType = appStateContext;
