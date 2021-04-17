import * as React from "react";
import { PageBaseComponent, PageBaseComponentProps } from "./PageBaseComponent";
import PhotoGallerySelectable from "../PhotoGallerySelectable";
import ContentContainer from "../ContentContainer";
import AppStateContext from "../../contexts/AppStateContext";
import PhotoService from "../../services/PhotoService";
import UploadHelper from "../../helpers/UploadHelper";
import ModalPhotoDetail from "../modals/ModalPhotoDetail";
import ModalConfirmation from "../modals/ModalConfirmation";
import ModalUploadProgress from "../modals/ModalUploadProgress";
import UploadButton from "../UploadButton";
import { IconRemove, IconImage } from "../Icons";
import { toast } from "react-toastify";
import UrlHelper from "../../helpers/UrlHelper";
import File from "../../models/File";
import GalleryPhoto from "../../models/GalleryPhoto";

const queryStringParamNamePhotoId = "photoId";

interface AlbumPageState {
	albumId: string,
	title: string,
	photos: GalleryPhoto[],
	selectedPhotos: string[],
	openedPhotoId: string | null,
	confirmDeleteAlbumOpen: boolean,
	confirmRemovePhotosOpen: boolean,
	uploadInProgress: boolean,
	uploadFiles: File[]
}

class AlbumPage extends PageBaseComponent<AlbumPageState> {

	constructor(props: PageBaseComponentProps) {
		super(props);

		this.state = {
			albumId: props.match.params.albumId,
			title: "",
			photos: [],
			selectedPhotos: [],
			openedPhotoId: null,
			confirmDeleteAlbumOpen: false,
			confirmRemovePhotosOpen: false,
			uploadInProgress: false,
			uploadFiles: []
		};
	}

	getHeaderActions(): JSX.Element | null {
		return <React.Fragment>
			{this.state.selectedPhotos.length === 1 && <button className="iconOnly" onClick={() => this.setSelectedPhotoAsAlbumCover()} title="Set album cover">
				<IconImage/>
			</button>}
			{this.state.selectedPhotos.length > 0 && <button className="iconOnly" onClick={() => this.onRemovePhotosClick()} title="Remove from album">
				<IconRemove/>
			</button>}
			{this.state.selectedPhotos.length === 0 && <button
				onClick={() => {
					const selectPhotosElement = document.getElementById("select-photos");
					if (selectPhotosElement) {
						selectPhotosElement.click();
					}
				}}
				title="Upload photos">
				Upload photos
			</button>}
		</React.Fragment>;
	}

	getHeaderContextMenu(): JSX.Element | null {
		return (<React.Fragment>
			{<button onClick={() => this.onDeleteAlbumClick()}>Delete album</button>}
		</React.Fragment>);
	}

	getTitle(): string {
		return "Album - " + this.state.title;
	}

	componentDidMount(): void {
		this.refreshPhotos();
		super.componentDidMount();
	}

	componentDidUpdate(prevProps: PageBaseComponentProps, prevState: AlbumPageState): void {
		// Open photo, if indicated as such by query string
		const queryStringPhotoId = UrlHelper.getQueryStringParamValue(location.search, queryStringParamNamePhotoId);
		if (this.state.openedPhotoId !== queryStringPhotoId) {
			this.setState({
				openedPhotoId: queryStringPhotoId
			});
		}

		super.componentDidUpdate(prevProps, prevState);
	}

	refreshPhotos(): void {
		PhotoService.getAlbum(this.state.albumId)
			.then((response) => {
				this.setState({
					title: response.title,
					photos: response.photos.map((photo): GalleryPhoto => {
						return {
							id: photo.id,
							src: PhotoService.baseUrl() + "/photo/" + photo.id + "/thumb",
							width: photo.width,
							height: photo.height
						};
					}),
					selectedPhotos: []
				});
			});
	}

	onDeleteAlbumClick(): void {
		this.setState({
			confirmDeleteAlbumOpen: true
		});
	}

	deleteAlbum(): void {
		const albumTitle = this.state.title;

		PhotoService.deleteAlbum(this.state.albumId)
			.then(() => {
				toast.info("Album '" + albumTitle + "' deleted.");
				this.context.history.push("/albums");
			})
			.catch(console.error);
	}

	onPhotoClicked(index: number): void {
		const photo = this.state.photos[index];
		this.context.history.push(document.location.pathname + "?photoId=" + photo.id);
	}

	setSelectedPhotoAsAlbumCover(): void {
		const _refreshPhotos = () => this.refreshPhotos();

		const photoId = this.state.selectedPhotos[0];

		PhotoService.updateAlbumCover(this.state.albumId, photoId)
			.then(() => {
				toast.info("Album cover updated.");
				_refreshPhotos();
			})
			.catch(console.error);
	}

	onRemovePhotosClick(): void {
		this.setState({
			confirmRemovePhotosOpen: true
		});
	}

	removeSelectedPhotosFromAlbum(): void {
		const fnRefreshPhotos = () => this.refreshPhotos();
		const fnCloseConfirmDialog = () => this.setState({ confirmRemovePhotosOpen: false });

		const selectedPhotos = this.state.selectedPhotos;
		const photoIds = this.state.photos.map(p => p.id);
		const remainingPhotosAfterRemoval = photoIds.filter(id => selectedPhotos.indexOf(id) === -1);

		PhotoService.updateAlbumPhotos(this.state.albumId, remainingPhotosAfterRemoval)
			.then(() => {
				toast.info("Photos removed.");
				fnCloseConfirmDialog();
				fnRefreshPhotos();
			})
			.catch(console.error);
	}

	onPhotoSelectedChange(photoId: string, selected: boolean): void {
		const selectedPhotos = this.state.selectedPhotos;

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

	onFilesDropped(event: React.DragEvent<HTMLElement>): void {
		event.preventDefault();
		if (!event.dataTransfer.files || event.dataTransfer.files.length === 0)
			return; // no files

		this.uploadFilesList(event.dataTransfer.files);
	}

	uploadFilesList (fileList: FileList): void {
		const albumId = this.state.albumId;
		let photoIds = this.state.photos.map(p => p.id);

		const files = UploadHelper.convertFileListToFileArrayForUploadDialog(fileList);

		const fnOnUploadFinished = (uploadedPhotoIds: string[]) => {
			this.setState({
				uploadInProgress: false,
				uploadFiles: []
			});

			const fnRefreshPhotos = () => this.refreshPhotos();

			if (uploadedPhotoIds && uploadedPhotoIds.length > 0) {
				toast.info("Upload finished.");

				photoIds = photoIds.concat(uploadedPhotoIds);
				PhotoService.updateAlbumPhotos(albumId, photoIds)
					.then(fnRefreshPhotos);
			}
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
				<div className="topBar">
					<h1>{this.state.title}</h1>
				</div>

				{!!this.state.title && this.state.photos.length === 0 &&
					<span className="centerText">This album has no photos.</span>
				}

				{this.state.photos.length > 0 && <PhotoGallerySelectable
					onClick={(_, target) => this.onPhotoClicked(target.index)}
					photos={this.state.photos}
					selectedItems={this.state.selectedPhotos}
					onPhotoSelectedChange={(photoId, selected) => this.onPhotoSelectedChange(photoId, selected)}/>
				}

				{this.state.openedPhotoId && <ModalPhotoDetail
					isOpen={!!this.state.openedPhotoId}
					photoId={this.state.openedPhotoId}
					onRequestClose={() => this.context.history.push(document.location.pathname + "?" + UrlHelper.removeQueryStringParam(document.location.search, queryStringParamNamePhotoId))}
				/>}

				<ModalConfirmation
					title="Delete album"
					isOpen={this.state.confirmDeleteAlbumOpen}
					onRequestClose={() => this.setState({confirmDeleteAlbumOpen: false})}
					onOkButtonClick={() => this.deleteAlbum()}
					okButtonText="Delete"
					confirmationText={"Album '" + this.state.title + "' will be deleted."}
					/>

				<ModalConfirmation
					title="Remove photos"
					isOpen={this.state.confirmRemovePhotosOpen}
					onRequestClose={() => this.setState({confirmRemovePhotosOpen: false})}
					onOkButtonClick={() => this.removeSelectedPhotosFromAlbum()}
					okButtonText="Remove"
					confirmationText={this.state.selectedPhotos.length + " photos will be removed from album '" + this.state.title + "'."}
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

AlbumPage.contextType = AppStateContext;
export default AlbumPage;