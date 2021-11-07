import * as React from "react";
import { PageBaseComponent, PageBaseComponentProps } from "./PageBaseComponent";
import PhotoGallery from "../misc/PhotoGallery";
import Content from "../layout/Content";
import appStateContext from "../../contexts/AppStateContext";
import ModalPhotoDetail from "../modals/ModalPhotoDetail";
import ModalConfirmation from "../modals/ModalConfirmation";
import UploadButton from "../misc/UploadButton";
import { IconDelete, IconUpload } from "../misc/Icons";
import { toast } from "react-toastify";
import UrlHelper from "../../helpers/UrlHelper";
import GalleryPhoto from "../../models/GalleryPhoto";
import AddPhotosToAlbumButton from "../buttons/AddPhotosToAlbumButton";
import uploadHelper from "../../helpers/UploadHelper";
import upholiService from "../../services/UpholiService";
import { PhotoMinimal } from "../../models/Photo";
import { AlbumNew } from "../../models/Album";

const queryStringParamNamePhotoId = "photoId";

interface LibraryPageState {
	photos: GalleryPhoto[],
	selectedPhotoIds: string[],
	openedPhotoId: string | null,
	confirmDeletePhotosOpen: boolean,
	albums: AlbumNew[]
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
		this.refreshPhotos = this.refreshPhotos.bind(this);

		this.state = {
			photos: [],
			selectedPhotoIds: [],
			openedPhotoId: null,
			confirmDeletePhotosOpen: false,
			albums: []
		};
	}

	getHeaderActions(): JSX.Element | null {
		return (<React.Fragment>
			{this.state.selectedPhotoIds.length === 0 && <button
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
		this.context.headerActions = this.getHeaderActions();
		this.context.headerContextMenu = this.getHeaderContextMenu();

		// Load the initial batch of photo thumbnails once all photo data has been fetched
		const anyPhotoLoaded = this.state.photos.some(p => !!p.src);
		if (!anyPhotoLoaded) {
			this.loadVisiblePhotos();

			// Workaround:
			// Call that function again after some time.
			// This is because the Gallery component still seems to move and re-fit the photos a bit after its render function has completed,
			// and I don't see any event for when it has fully finished rendering.
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
					upholiService.getPhotoThumbnailImageSrc(photoInfo.id)
						.then(src => {
							this.setState(previousState => {
								const photo = previousState.photos.find(p => p.id === statePhoto.id);
								if (photo) {
									photo.src = src;
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
		upholiService.deletePhotos(this.state.selectedPhotoIds)
			.then(() => {
				const remainingPhotos = this.state.photos
					.filter(p => this.state.selectedPhotoIds.indexOf(p.id) === -1);

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

		this.uploadFilesList(event.dataTransfer.files);
	}

	uploadFilesList(fileList: FileList): void {
		const fnOnUploadFinished = () => {
			this.refreshPhotos();
			toast.info("Upload finished.");
		};

		uploadHelper.uploadPhotos(fileList).then(() => {
			fnOnUploadFinished();
		});
	}

	render(): React.ReactNode {
		return (
			<Content onDrop={(event) => this.onFilesDropped(event)}>
				<PhotoGallery photos={this.state.photos} onClick={(_, target) => this.onPhotoClicked(target.index)} selectedItems={this.state.selectedPhotoIds} onPhotoSelectedChange={(photoId, selected) => this.onPhotoSelectedChange(photoId, selected)} />

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

				{/* Hidden upload button triggered by the button in action bar. This allos me to write simpler CSS to style the action buttons. */}
				<UploadButton className="hidden" onSubmit={(files) => this.uploadFilesList(files)}/>
			</Content>
		);
	}
}

LibraryPage.contextType = appStateContext;
export default LibraryPage;