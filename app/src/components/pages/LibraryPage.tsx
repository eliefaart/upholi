import * as React from "react";
import { PageBaseComponent, PageBaseComponentProps } from "./PageBaseComponent";
import PhotoGallerySelectable from "../PhotoGallerySelectable";
import ContentContainer from "../ContentContainer";
import PhotoService from "../../services/PhotoService";
import UploadHelper from "../../helpers/UploadHelper";
import AppStateContext from "../../contexts/AppStateContext";
import ModalPhotoDetail from "../modals/ModalPhotoDetail";
import ModalConfirmation from "../modals/ModalConfirmation";
import ModalUploadProgress from "../modals/ModalUploadProgress";
import UploadButton from "../UploadButton";
import { IconDelete } from "../Icons";
import { toast } from "react-toastify";
import UrlHelper from "../../helpers/UrlHelper";
import Photo from "../../models/Photo";
import File from "../../models/File";
import GalleryPhoto from "../../models/GalleryPhoto";
import AlbumInfo from "../../models/AlbumInfo";
import AddPhotosToAlbumButton from "../Buttons/AddPhotosToAlbumButton";
import uploadHelper from "../../helpers/UploadHelperNew";
import upholiService, { PhotoMinimal } from "../../services/UpholiService";

const queryStringParamNamePhotoId = "photoId";

interface LibraryPageState {
	photos: GalleryPhoto[],
	selectedPhotoIds: string[],
	openedPhotoId: string | null,
	confirmDeletePhotosOpen: boolean,
	albums: AlbumInfo[],
	uploadInProgress: boolean,
	uploadFiles: File[]
}

class LibraryPage extends PageBaseComponent<LibraryPageState> {

	photos: PhotoMinimal[];
	waitingForPhotoCheck: boolean;
	photoCheckQueued: boolean;
	onScroll: () => void;

	constructor(props: PageBaseComponentProps) {
		super(props);

		// Contains all user's photos, but this is not the viewmodel of the Gallery
		this.photos = [];

		this.waitingForPhotoCheck = false;
		this.photoCheckQueued = false;
		this.onScroll = () => {
			// No need to check check photo visibility every time the scroll event fires,
			// because it may fire many times per second. Limiting to every 50ms is more than enough.
			const msTimeBetweenChecks = 50;

			if (!this.waitingForPhotoCheck) {
				this.photoCheckQueued = true;
				setTimeout(() => {
					this.loadVisiblePhotos();
					this.photoCheckQueued = false;
				}, msTimeBetweenChecks);
			}
		};

		this.resetSelection = this.resetSelection.bind(this);

		this.state = {
			photos: [],
			selectedPhotoIds: [],
			openedPhotoId: null,
			confirmDeletePhotosOpen: false,
			albums: [],
			uploadInProgress: false,
			uploadFiles: []
		};
	}

	getHeaderActions(): JSX.Element | null {
		return (<React.Fragment>
			{this.state.selectedPhotoIds.length === 0 && <button
				onClick={() => {
					const element = document.getElementById("select-photos");
					if (element) {
						element.click();
					}
				}}
				title="Upload photos">
				Upload photos
			</button>}
			<AddPhotosToAlbumButton
				selectedPhotoIds={this.state.selectedPhotoIds}
				onSelectionAddedToAlbum={this.resetSelection}/>
			{this.state.selectedPhotoIds.length > 0 && <button className="iconOnly" onClick={() => this.onClickDeletePhotos()} title="Delete photos">
				<IconDelete/>
			</button>}
		</React.Fragment>);
	}

	getTitle(): string {
		return "Library";
	}

	componentDidMount(): void {
		this.refreshPhotos();

		const contentElement = document.getElementById("content");
		if (contentElement) {
			contentElement.addEventListener("scroll", this.onScroll);
		}

		super.componentDidMount();
	}

	componentDidUpdate(prevProps: PageBaseComponentProps, prevState: LibraryPageState): void {
		// Load the initial batch of photo thumbnails once all photo data has been fetched
		if (prevState.photos.length === 0 && this.state.photos.length > 0) {
			this.loadVisiblePhotos();

			// Workaround:
			// Call that function again after some time.
			// This is because the Gallery component still seems to move and re-fit the photos a bit after its render function has completed,
			// and I don't see any event for when it is fully finished rendering.
			setTimeout(() => this.loadVisiblePhotos(), 500);
		}

		// Remove onscroll event handler from body once all photos have been loaded
		if (this.state.photos.length > 0) {
			const nPhotosNotYetLoaded = this.state.photos.filter(photo => photo.src === "").length;
			if (nPhotosNotYetLoaded === 0) {
				const contentElement = document.getElementById("content");
				if (contentElement) {
					contentElement.removeEventListener("scroll", this.onScroll);
				}
			}
		}

		// Open photo, if indicated as such by query string
		const queryStringPhotoId = UrlHelper.getQueryStringParamValue(location.search, queryStringParamNamePhotoId);
		if (this.state.openedPhotoId !== queryStringPhotoId) {
			this.setState({
				openedPhotoId: queryStringPhotoId
			});
		}

		super.componentDidUpdate(prevProps, prevState);
	}

	/**
	 * Get info of all photos in user's library
	 */
	refreshPhotos(): void {
		const fnSetPhotos = (photos: PhotoMinimal[]) => {
			this.photos = photos;

			const galleryPhotos: GalleryPhoto[] = [];
			for (const photo of photos) {
				galleryPhotos.push({
					id: photo.id,
					width: photo.width,
					height: photo.height,
					src: ""
				});
			}

			this.setState({
				photos: galleryPhotos
			});
		};

		upholiService.getPhotos()
			.then(fnSetPhotos)
			.catch(console.error);
	}

	/**
	 * Load all photo thumbnails that are visible in the viewport
	 */
	loadVisiblePhotos(): void {
		// Function that checks if given element is at least partially visible within viewport
		const fnElementIsInViewport = (element: HTMLElement) => {
			if (!element)  {
				return false;
			}
			else {
				const myElementHeight = element.offsetHeight;
				const myElementWidth = element.offsetWidth;
				const bounding = element.getBoundingClientRect();

				return bounding.top >= -myElementHeight
					&& bounding.left >= -myElementWidth
					&& bounding.right <= (window.innerWidth || document.documentElement.clientWidth) + myElementWidth
					&& bounding.bottom <= (window.innerHeight || document.documentElement.clientHeight) + myElementHeight;
			}
		};

		const statePhotos = this.state.photos;

		for (const statePhoto of statePhotos) {
			const photoHasBeenLoaded = statePhoto.src !== "";
			if (!photoHasBeenLoaded) {
				const photoInfo = this.photos.find(p => p.id === statePhoto.id);
				const photoElement = document.getElementById(statePhoto.id);

				if (photoElement && photoInfo && fnElementIsInViewport(photoElement)) {

					//statePhoto.src = PhotoService.getThumbUrl(photoInfo.id);

					upholiService.getPhotoThumbnailBase64(photoInfo.id)
						.then(base64 => {
							this.setState(previousState => {
								const photo = previousState.photos.find(p => p.id === statePhoto.id);
								if (photo) {
									photo.src = `data:image/png;base64,${base64}`;
								}

								return {
									photos: previousState.photos
								};
							});
						});
				}
			}
		}
	}

	resetSelection(): void {
		this.setState({
			selectedPhotoIds: []
		});
	}

	onClickDeletePhotos(): void {
		this.setState({
			confirmDeletePhotosOpen: true
		});
	}

	deleteSelectedPhotos(): void {
		const promises: Promise<void>[] = [];
		for (const id of this.state.selectedPhotoIds) {
			promises.push(upholiService.deletePhoto(id));
		}

		Promise.all(promises)
			.then(() => {
				const remainingPhotos = this.state.photos.filter(p =>
					this.state.selectedPhotoIds.indexOf(p.id) === -1);

				this.setState({
					photos: remainingPhotos,
					selectedPhotoIds: [],
					confirmDeletePhotosOpen: false
				});
			});
	}

	onPhotoSelectedChange(photoId: string, selected: boolean): void {
		const selectedPhotos = this.state.selectedPhotoIds;

		if (selected) {
			selectedPhotos.push(photoId);
		} else {
			const index = selectedPhotos.indexOf(photoId);
			if (index > -1) {
				selectedPhotos.splice(index, 1);
			}
		}

		this.setState({
			selectedPhotoIds: selectedPhotos
		});
	}

	onSelectionChange(selectedPhotos: string[]): void {
		this.setState({
			selectedPhotoIds: selectedPhotos
		});
	}

	onPhotoClicked(index: number): void {
		const photo = this.state.photos[index];
		this.context.history.push(document.location.pathname + "?photoId=" + photo.id);
	}

	onFilesDropped(event: React.DragEvent<HTMLElement>): void {
		event.preventDefault();
		if (!event.dataTransfer.files || event.dataTransfer.files.length === 0)
			return; // no files

		uploadHelper.uploadPhotos(event.dataTransfer.files, (progress) => console.log(progress[0].status))
			.then(() => {
				console.log("Upload finished?");
			});

		// this.uploadFilesList(event.dataTransfer.files);
	}

	uploadFilesList(fileList: FileList): void {
		const files = UploadHelper.convertFileListToFileArrayForUploadDialog(fileList);
		const fnOnUploadFinished = () => {
			this.setState({
				uploadInProgress: false,
				uploadFiles: []
			});
			this.refreshPhotos();
			toast.info("Upload finished.");
		};
		const fnUpdateFileUploadState = (file: globalThis.File, newState: string) => {
			const stateFile = files.find(f => f.name === file.name);
			if (stateFile) {
				stateFile.status = newState;

				this.setState({
					uploadFiles: files
				});
			}
		};

		PhotoService.uploadPhotos(fileList, fnUpdateFileUploadState)
			.then(fnOnUploadFinished)
			.catch(console.error);

		this.setState({
			uploadInProgress: true,
			uploadFiles: files
		});
	}

	render(): React.ReactNode {
		return (
			<ContentContainer onDrop={(event) => this.onFilesDropped(event)}>
				<PhotoGallerySelectable photos={this.state.photos} onClick={(_, target) => this.onPhotoClicked(target.index)} selectedItems={this.state.selectedPhotoIds} onPhotoSelectedChange={(photoId, selected) => this.onPhotoSelectedChange(photoId, selected)} />

				{this.state.openedPhotoId && <ModalPhotoDetail
					isOpen={!!this.state.openedPhotoId}
					photoId={this.state.openedPhotoId}
					onRequestClose={() => this.context.history.push(document.location.pathname + "?" + UrlHelper.removeQueryStringParam(document.location.search, queryStringParamNamePhotoId))}
				/>}

				<ModalConfirmation
					title="Delete photos"
					isOpen={this.state.confirmDeletePhotosOpen}
					onRequestClose={() => this.setState({confirmDeletePhotosOpen: false})}
					onOkButtonClick={() => this.deleteSelectedPhotos()}
					okButtonText="Delete"
					confirmationText={this.state.selectedPhotoIds.length + " photos will be deleted."}
					/>

				<ModalUploadProgress
					isOpen={this.state.uploadInProgress}
					onRequestClose={() => this.setState({uploadInProgress: false})}
					files={this.state.uploadFiles}
					/>

				{/* Hidden upload button triggered by the button in action bar. This allos me to write simpler CSS to style the action buttons. */}
				<UploadButton className="hidden" onSubmit={(files) => this.uploadFilesList(files)}/>
			</ContentContainer>
		);
	}
}

LibraryPage.contextType = AppStateContext;
export default LibraryPage;