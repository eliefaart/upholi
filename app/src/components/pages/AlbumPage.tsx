import * as React from "react";
import { PageBaseComponent, PageBaseComponentProps } from "./PageBaseComponent";
import PhotoGallerySelectable from "../PhotoGallerySelectable";
import ContentContainer from "../ContentContainer";
import appStateContext from "../../contexts/AppStateContext";
import ModalPhotoDetail from "../modals/ModalPhotoDetail";
import ModalConfirmation from "../modals/ModalConfirmation";
import UploadButton from "../UploadButton";
import { IconRemove, IconImage, IconUpload, IconShare } from "../Icons";
import { toast } from "react-toastify";
import UrlHelper from "../../helpers/UrlHelper";
import GalleryPhoto from "../../models/GalleryPhoto";
import ModalEditAlbum from "../modals/ModalEditAlbum";
import Album from "../../models/Album";
import AddPhotosToAlbumButton from "../Buttons/AddPhotosToAlbumButton";
import upholiService from "../../services/UpholiService";
import uploadHelper from "../../helpers/UploadHelper";
import ModalSharingOptions from "../modals/ModalSharingOptions";
import { SharingOptions } from "../../models/SharingOptions";

const queryStringParamNamePhotoId = "photoId";

interface AlbumPageState {
	albumId: string,
	album: Album | null,
	sharingOptions: {
		token: string
	},
	galleryPhotos: GalleryPhoto[],
	selectedPhotoIds: string[],
	openedPhotoId: string | null,
	editAlbumOpen: boolean,
	sharingOptionsOpen: boolean,
	confirmDeleteAlbumOpen: boolean,
	confirmRemovePhotosOpen: boolean
}

class AlbumPage extends PageBaseComponent<AlbumPageState> {

	constructor(props: PageBaseComponentProps) {
		super(props);

		this.onEditAlbumClick = this.onEditAlbumClick.bind(this);
		this.onDeleteAlbumClick = this.onDeleteAlbumClick.bind(this);
		this.resetSelection = this.resetSelection.bind(this);
		this.updateSharingOptions = this.updateSharingOptions.bind(this);

		this.state = {
			albumId: props.match.params.albumId,
			album: null,
			sharingOptions: {
				token: ""
			},
			galleryPhotos: [],
			selectedPhotoIds: [],
			openedPhotoId: null,
			editAlbumOpen: false,
			sharingOptionsOpen: false,
			confirmDeleteAlbumOpen: false,
			confirmRemovePhotosOpen: false
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
				className="iconOnly"
				onClick={() => {
					const selectPhotosElement = document.getElementById("select-photos");
					if (selectPhotosElement) {
						selectPhotosElement.click();
					}
				}}
				title="Upload photos">
					<IconUpload/>
			</button>}
			{this.state.selectedPhotoIds.length === 0 && <button
				className="iconOnly"
				onClick={() => this.setState({sharingOptionsOpen: true})}
				title="Sharing options">
					<IconShare/>
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
		upholiService.getAlbum(this.state.albumId)
			.then(album => {
				this.setState({
					album,
					galleryPhotos: album.photos.map((photo): GalleryPhoto => {
						return {
							id: photo.id,
							src: "",
							width: photo.width,
							height: photo.height
						};
					}),
					selectedPhotoIds: []
				});

				for (const albumPhoto of album.photos) {
					upholiService.getPhotoThumbnailImageSrc(albumPhoto.id)
						.then(src => {
							this.setState(previousState => {
								const albumPhotoToUpdate = previousState.galleryPhotos.find(p => p.id === albumPhoto.id);
								if (albumPhotoToUpdate) {
									albumPhotoToUpdate.src = src;
								}

								return {
									galleryPhotos: previousState.galleryPhotos
								};
							});
						});
				}
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
		const photoId = this.state.selectedPhotoIds[0];

		upholiService.updateAlbumCover(this.state.albumId, photoId)
			.then(() => {
				toast.info("Album cover updated.");
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
		const fnOnUploadFinished = () => {
			this.refreshPhotos();
			toast.info("Upload finished.");
		};

		uploadHelper.uploadPhotos(fileList).then((queue) => {
			if (this.state.album) {
				const photoIds = queue
					.map(file => file.uploadedPhotoId || "")
					.filter(id => !!id);
				upholiService.addPhotosToAlbum(this.state.album.id, photoIds)
					.finally(fnOnUploadFinished);
			}
		});
	}

	updateSharingOptions(options: SharingOptions): void {
		// derive key from password.
		// create 'share' in album
		// create 'share' in each album's photo
		upholiService.updateAlbumSharingOptions(this.state.albumId, options)
			.then(token => {
				this.setState({
					sharingOptions: {
						token
					}
				});
			});
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

					<ModalSharingOptions
						shareUrl={document.location.origin + "/s/" + this.state.sharingOptions.token}
						isOpen={this.state.sharingOptionsOpen}
						onOkButtonClick={() => null}
						onRequestClose={() => this.setState({sharingOptionsOpen: false})}
						onSharingOptionsUpdated={this.updateSharingOptions}
						/>

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

					{/* Hidden upload button triggered by the button in action bar. This allos me to write simpler CSS to style the action buttons. */}
					<UploadButton className="hidden" onSubmit={(files) => this.uploadFilesList(files)}/>
				</ContentContainer>
			);
		}
	}
}

AlbumPage.contextType = appStateContext;
export default AlbumPage;