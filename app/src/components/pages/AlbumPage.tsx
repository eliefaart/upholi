import * as React from "react";
import { PageBaseComponent, PageBaseComponentProps } from "./PageBaseComponent";
import PhotoGallerySelectable from "../PhotoGallerySelectable";
import ContentContainer from "../ContentContainer";
import AppStateContext from "../../contexts/AppStateContext";
import PhotoService from "../../services/PhotoService";
import ModalPhotoDetail from "../modals/ModalPhotoDetail";
import ModalConfirmation from "../modals/ModalConfirmation";
import ModalUploadProgress from "../modals/ModalUploadProgress";
import UploadButton from "../UploadButton";
import { IconRemove, IconImage } from "../Icons";
import { toast } from "react-toastify";
import UrlHelper from "../../helpers/UrlHelper";
import File from "../../models/File";
import GalleryPhoto from "../../models/GalleryPhoto";
import ModalEditAlbum from "../modals/ModalEditAlbum";
import Album from "../../models/Album";
import AddPhotosToAlbumButton from "../Buttons/AddPhotosToAlbumButton";
import upholiService from "../../services/UpholiService";

const queryStringParamNamePhotoId = "photoId";

interface AlbumPageState {
	albumId: string,
	album: Album | null,
	galleryPhotos: GalleryPhoto[],
	selectedPhotoIds: string[],
	openedPhotoId: string | null,
	editAlbumOpen: boolean,
	confirmDeleteAlbumOpen: boolean,
	confirmRemovePhotosOpen: boolean,
	uploadInProgress: boolean,
	uploadFiles: File[]
}

class AlbumPage extends PageBaseComponent<AlbumPageState> {

	constructor(props: PageBaseComponentProps) {
		super(props);

		this.onEditAlbumClick = this.onEditAlbumClick.bind(this);
		this.onDeleteAlbumClick = this.onDeleteAlbumClick.bind(this);
		this.resetSelection = this.resetSelection.bind(this);

		this.state = {
			albumId: props.match.params.albumId,
			album: null,
			galleryPhotos: [],
			selectedPhotoIds: [],
			openedPhotoId: null,
			editAlbumOpen: false,
			confirmDeleteAlbumOpen: false,
			confirmRemovePhotosOpen: false,
			uploadInProgress: false,
			uploadFiles: []
		};
	}

	getHeaderActions(): JSX.Element | null {
		return <React.Fragment>
			{this.state.selectedPhotoIds.length === 1 && <button className="iconOnly" onClick={() => this.setSelectedPhotoAsAlbumCover()} title="Set album cover">
				<IconImage/>
			</button>}
			<AddPhotosToAlbumButton
				selectedPhotoIds={this.state.selectedPhotoIds}
				onSelectionAddedToAlbum={this.resetSelection}/>
			{this.state.selectedPhotoIds.length > 0 && <button className="iconOnly" onClick={() => this.onRemovePhotosClick()} title="Remove from album">
				<IconRemove/>
			</button>}
			{this.state.selectedPhotoIds.length === 0 && <button
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
			{<button onClick={this.onEditAlbumClick}>Edit album</button>}
			{<button onClick={this.onDeleteAlbumClick}>Delete album</button>}
		</React.Fragment>);
	}

	getTitle(): string {
		return "Album - " + this.state.album?.title;
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
			.then((album) => {
				this.setState({
					album,
					galleryPhotos: album.photos.map((photo): GalleryPhoto => {
						return {
							id: photo.id,
							src: PhotoService.baseUrl() + "/photo/" + photo.id + "/thumb",
							width: photo.width,
							height: photo.height
						};
					}),
					selectedPhotoIds: []
				});
			});
	}

	resetSelection(): void {
		this.setState({
			selectedPhotoIds: []
		});
	}

	onDeleteAlbumClick(): void {
		this.setState({
			confirmDeleteAlbumOpen: true
		});
	}

	onEditAlbumClick(): void {
		this.setState({
			editAlbumOpen: true
		});
	}

	deleteAlbum(): void {
		const albumTitle = this.state.album?.title;

		upholiService.deleteAlbum(this.state.albumId)
			.then(() => {
				toast.info("Album '" + albumTitle + "' deleted.");
				this.context.history.push("/albums");
			})
			.catch(console.error);
	}

	onPhotoClicked(index: number): void {
		const photo = this.state.galleryPhotos[index];
		this.context.history.push(document.location.pathname + "?photoId=" + photo.id);
	}

	setSelectedPhotoAsAlbumCover(): void {
		const _refreshPhotos = () => this.refreshPhotos();

		const photoId = this.state.selectedPhotoIds[0];

		upholiService.updateAlbumCover(this.state.albumId, photoId)
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

		upholiService.removePhotosFromAlbum(this.state.albumId, this.state.selectedPhotoIds)
			.then(() => {
				toast.info("Photos removed.");
				fnCloseConfirmDialog();
				fnRefreshPhotos();
			})
			.catch(console.error);
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

	onFilesDropped(event: React.DragEvent<HTMLElement>): void {
		event.preventDefault();
		if (!event.dataTransfer.files || event.dataTransfer.files.length === 0)
			return; // no files

		this.uploadFilesList(event.dataTransfer.files);
	}

	uploadFilesList (fileList: FileList): void {
		console.warn("Todo: implement AlbumPage.uploadFilesList");
		// const albumId = this.state.albumId;
		// let photoIds = this.state.galleryPhotos.map(p => p.id);

		// const files = UploadHelper.convertFileListToFileArrayForUploadDialog(fileList);

		// const fnOnUploadFinished = (uploadedPhotoIds: string[]) => {
		// 	this.setState({
		// 		uploadInProgress: false,
		// 		uploadFiles: []
		// 	});

		// 	const fnRefreshPhotos = () => this.refreshPhotos();

		// 	if (uploadedPhotoIds && uploadedPhotoIds.length > 0) {
		// 		toast.info("Upload finished.");

		// 		photoIds = photoIds.concat(uploadedPhotoIds);
		// 		PhotoService.updateAlbumPhotos(albumId, photoIds)
		// 			.then(fnRefreshPhotos);
		// 	}
		// };
		// const fnUpdateFileUploadState = (file: globalThis.File, newState: string) => {
		// 	const stateFile = files.find(f => f.name === file.name);
		// 	if (stateFile) {
		// 		stateFile.status = newState;

		// 		this.setState({
		// 			uploadFiles: files
		// 		});
		// 	}
		// };

		// PhotoService.uploadPhotos(fileList, fnUpdateFileUploadState)
		// 	.then(fnOnUploadFinished)
		// 	.catch(console.error);

		// this.setState({
		// 	uploadInProgress: true,
		// 	uploadFiles: files
		// });
	}

	render(): React.ReactNode {
		if (!this.state.album) {
			return null;
		}
		else {
			return (
				<ContentContainer onDrop={(event) => this.onFilesDropped(event)}>
					<div className="topBar">
						<h1>{this.state.album.title}</h1>
					</div>

					{!!this.state.album.title && this.state.galleryPhotos.length === 0 &&
						<span className="centerText">This album has no photos.</span>
					}

					{this.state.galleryPhotos.length > 0 && <PhotoGallerySelectable
						onClick={(_, target) => this.onPhotoClicked(target.index)}
						photos={this.state.galleryPhotos}
						selectedItems={this.state.selectedPhotoIds}
						onPhotoSelectedChange={(photoId, selected) => this.onPhotoSelectedChange(photoId, selected)}/>
					}

					{this.state.openedPhotoId && <ModalPhotoDetail
						isOpen={!!this.state.openedPhotoId}
						photoId={this.state.openedPhotoId}
						onRequestClose={() => this.context.history.push(document.location.pathname + "?" + UrlHelper.removeQueryStringParam(document.location.search, queryStringParamNamePhotoId))}
					/>}

					<ModalEditAlbum
						isOpen={this.state.editAlbumOpen}
						onRequestClose={() => this.setState({editAlbumOpen: false})}
						album={this.state.album}/>

					<ModalConfirmation
						title="Delete album"
						isOpen={this.state.confirmDeleteAlbumOpen}
						onRequestClose={() => this.setState({confirmDeleteAlbumOpen: false})}
						onOkButtonClick={() => this.deleteAlbum()}
						okButtonText="Delete"
						confirmationText={"Album '" + this.state.album.title + "' will be deleted."}
						/>

					<ModalConfirmation
						title="Remove photos"
						isOpen={this.state.confirmRemovePhotosOpen}
						onRequestClose={() => this.setState({confirmRemovePhotosOpen: false})}
						onOkButtonClick={() => this.removeSelectedPhotosFromAlbum()}
						okButtonText="Remove"
						confirmationText={this.state.selectedPhotoIds.length + " photos will be removed from album '" + this.state.album.title + "'."}
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
}

AlbumPage.contextType = AppStateContext;
export default AlbumPage;