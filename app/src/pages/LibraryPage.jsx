import React from "react";
import PageBaseComponent from "../components/PageBaseComponent.jsx";
import PhotoGallerySelectable from "../components/PhotoGallerySelectable.jsx";
import ContentContainer from "../components/ContentContainer.tsx";
import PhotoService from "../services/PhotoService.ts";
import UploadHelper from "../helpers/UploadHelper.ts";
import AppStateContext from "../contexts/AppStateContext.ts";
import ModalPhotoDetail from "../components/ModalPhotoDetail.tsx";
import ModalConfirmation from "../components/ModalConfirmation.tsx";
import ModalUploadProgress from "../components/ModalUploadProgress.tsx";
import ModalCreateAlbum from "../components/ModalCreateAlbum.tsx";
import ModalAddToAlbum from "../components/ModalAddToAlbum.tsx";
import UploadButton from "../components/UploadButton.tsx";
import { IconDelete, IconAddToAlbum } from "../components/Icons.tsx";
import { toast } from "react-toastify";
import UrlHelper from "../helpers/UrlHelper.ts";

const queryStringParamNamePhotoId = "photoId";

class LibraryPage extends PageBaseComponent {

	constructor(props) {
		super(props);

		// Contains all user's photos, but this is not the viewmodel of the Gallery
		this.photos = [];

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
		}

		this.state = {
			photos: [],
			selectedPhotos: [],
			openedPhotoId: null,
			newAlbumDialogOpen: false,
			confirmDeletePhotosOpen: false,
			addPhotosToAlbumDialogOpen: false,
			albums: [],
			uploadInProgress: false,
			uploadFiles: []
		};
	}

	getHeaderActions() {
		return (<React.Fragment>
			{this.state.selectedPhotos.length === 0 && <button onClick={() => document.getElementById("select-photos").click()} title="Upload photos">
				Upload
			</button>}
			{this.state.selectedPhotos.length > 0 && <button className="iconOnly" onClick={() => this.onClickAddSelectedPhotosToAlbum(this.state.selectedPhotos)} title="Add to album">
				<IconAddToAlbum/>
			</button>}
			{this.state.selectedPhotos.length > 0 && <button className="iconOnly" onClick={() => this.onClickDeletePhotos()} title="Delete photos">
				<IconDelete/>
			</button>}
		</React.Fragment>);
	}

	getTitle() {
		return "Library";
	}

	componentDidMount() {
		this.refreshPhotos();

		document.getElementById("content").addEventListener("scroll", this.onScroll);

		super.componentDidMount();
	}

	componentDidUpdate(prevProps, prevState) {
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
				document.getElementById("content").removeEventListener("scroll", this.onScroll);
			}
		}

		// Open photo, if indicated as such by query string
		const queryStringPhotoId = UrlHelper.getQueryStringParamValue(location.search, queryStringParamNamePhotoId);
		if (this.state.openedPhotoId !== queryStringPhotoId) {
			this.setState({
				openedPhotoId: queryStringPhotoId
			});
		}

		super.componentDidUpdate();
	}

	/**
	 * Get info of all photos in user's library
	 */
	refreshPhotos() {
		const fnSetPhotos = (photos) => {
			this.photos = photos;

			const galleryPhotos = [];
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

		PhotoService.getPhotos()
			.then(fnSetPhotos)
			.catch(console.error);
	}

	/**
	 * Load all photo thumbnails that are visible in the viewport
	 */
	loadVisiblePhotos() {
		// Function that checks if given element is at least partially visible within viewport
		const fnElementIsInViewport = (element) => {
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
		let anyPhotosLoaded = false;

		for (const statePhoto of statePhotos) {
			const photoHasBeenLoaded = statePhoto.src !== "";
			if (!photoHasBeenLoaded) {
				const photoInfo = this.photos.find(p => p.id === statePhoto.id);
				const photoElement = document.getElementById(statePhoto.id);

				if (photoElement && photoInfo && fnElementIsInViewport(photoElement)) {
					anyPhotosLoaded = true;
					statePhoto.src = PhotoService.getThumbUrl(photoInfo.id);
				}
			}
		}

		if (anyPhotosLoaded) {
			this.setState({
				photos: statePhotos
			});
		}
	}

	resetSelection() {
		this.setState({
			selectedPhotos: []
		});
	}

	onClickDeletePhotos() {
		this.setState({
			confirmDeletePhotosOpen: true
		});
	}

	deleteSelectedPhotos() {
		PhotoService.deletePhotos(this.state.selectedPhotos)
			.then(() => {
				let remainingPhotos = this.state.photos.filter(p =>
					this.state.selectedPhotos.indexOf(p.id) === -1);

				this.setState({
					photos: remainingPhotos,
					selectedPhotos: [],
					confirmDeletePhotosOpen: false
				});

				toast.info("Photos deleted.");
			})
			.catch(console.error);
	}

	onClickAddSelectedPhotosToAlbum() {
		let _this = this;

		PhotoService.getAlbums()
			.then((albums) => {
				_this.setState({
					albums: albums,
					addPhotosToAlbumDialogOpen: true
				});
			});
	}

	addSelectedPhotosToAlbum(albumId) {
		let _this = this;

		PhotoService.addPhotosToAlbum(albumId, this.state.selectedPhotos)
			.then(() => {
				_this.setState({
					selectedPhotos: [],
					addPhotosToAlbumDialogOpen: false
				})
				toast.info("Photos added to album.");
			})
			.catch(console.error);
	}

	onPhotoSelectedChange(photoId, selected) {
		let selectedPhotos = this.state.selectedPhotos;

		if (selected) {
			selectedPhotos.push(photoId);
		} else {
			const index = selectedPhotos.indexOf(photoId);
			if (index > -1) {
				selectedPhotos.splice(index, 1);
			}
		}

		this.setState({
			selectedPhotos: selectedPhotos
		});
	}

	onSelectionChange(selectedPhotos) {
		this.setState({
			selectedPhotos: selectedPhotos
		});
	}

	createNewAlbum() {
		this.setState({
			newAlbumDialogOpen: true
		})
	}

	onPhotoClicked(event, target) {
		let photo = this.state.photos[target.index];
		this.context.history.push(document.location.pathname + "?photoId=" + photo.id);
	}

	onFilesDropped(event) {
		event.preventDefault();
		if (!event.dataTransfer.files || event.dataTransfer.files.length === 0)
			return; // no files

		this.uploadFilesList(event.dataTransfer.files);
	}

	uploadFilesList(filesList) {
		let files = UploadHelper.convertFileListToFileArrayForUploadDialog(filesList);
		let fnOnUploadFinished = () => {
			this.setState({
			 	uploadInProgress: false,
			 	uploadFiles: []
			});
			this.refreshPhotos();
			toast.info("Upload finished.");
		};
		let fnUpdateFileUploadState = (file, newState) => {
			let stateFile = files.find(f => f.name === file.name);
			stateFile.status = newState;

			this.setState({
				uploadFiles: files
			});
		};

		PhotoService.uploadPhotos(filesList, fnUpdateFileUploadState)
			.then(fnOnUploadFinished)
			.catch(console.error);

		this.setState({
			uploadInProgress: true,
			uploadFiles: files
		});
	}

	render() {
		return (
			<ContentContainer onDrop={(event) => this.onFilesDropped(event)}>
				<PhotoGallerySelectable photos={this.state.photos} onClick={(event, target) => this.onPhotoClicked(event, target)} selectedItems={this.state.selectedPhotos} onPhotoSelectedChange={(photoId, selected) => this.onPhotoSelectedChange(photoId, selected)} />

				<ModalPhotoDetail
					isOpen={!!this.state.openedPhotoId}
					photoId={this.state.openedPhotoId}
					onRequestClose={() => this.context.history.push(document.location.pathname + "?" + UrlHelper.removeQueryStringParam(document.location.search, queryStringParamNamePhotoId))}
					/>

				<ModalCreateAlbum
					isOpen={this.state.newAlbumDialogOpen}
					onRequestClose={() => this.setState({newAlbumDialogOpen: false})}
					createWithPhotoIds={this.state.selectedPhotos}
					/>

				<ModalAddToAlbum
					isOpen={this.state.addPhotosToAlbumDialogOpen}
					onRequestClose={() => this.setState({addPhotosToAlbumDialogOpen: false})}
					onClickNewAlbum={() => this.createNewAlbum()}
					onClickExistingAlbum={(album) => this.addSelectedPhotosToAlbum(album.id)}
					/>

				<ModalConfirmation
					title="Delete photos"
					isOpen={this.state.confirmDeletePhotosOpen}
					onRequestClose={() => this.setState({confirmDeletePhotosOpen: false})}
					onOkButtonClick={() => this.deleteSelectedPhotos(this.state.selectedPhotos)}
					okButtonText="Delete"
					confirmationText={this.state.selectedPhotos.length + " photos will be deleted."}
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