import * as React from "react";
import { PageBaseComponent, PageBaseComponentProps } from "./PageBaseComponent";
import ContentContainer from "../ContentContainer";
import appStateContext from "../../contexts/AppStateContext";
import ModalConfirmation from "../modals/ModalConfirmation";
import UploadButton from "../misc/UploadButton";
import { IconRemove, IconImage, IconUpload, IconShare } from "../Icons";
import { toast } from "react-toastify";
import UrlHelper from "../../helpers/UrlHelper";
import GalleryPhoto from "../../models/GalleryPhoto";
import ModalEditAlbum from "../modals/ModalEditAlbum";
import { Album } from "../../models/Album";
import AddPhotosToAlbumButton from "../buttons/AddPhotosToAlbumButton";
import upholiService from "../../services/UpholiService";
import uploadHelper from "../../helpers/UploadHelper";
import ModalSharingOptions from "../modals/ModalSharingOptions";
import { SharingOptions } from "../../models/SharingOptions";
import AlbumView from "../AlbumView";
import { Share } from "../../models/Share";

const queryStringParamNamePhotoId = "photoId";

interface AlbumPageState {
	albumId: string,
	album: Album | null,
	share: Share | null,
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
			share: null,
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
		this.refreshAlbum();
		this.refreshShare();
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

	refreshAlbum(): void {
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
			});
	}

	refreshShare(): void {
		upholiService.findAlbumShare(this.state.albumId)
			.then(share => {
				this.setState({ share });
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
				this.resetSelection();
			})
			.catch(console.error);
	}

	onRemovePhotosClick(): void {
		this.setState({
			confirmRemovePhotosOpen: true,
			selectedPhotoIds: []
		});
	}

	removeSelectedPhotosFromAlbum(): void {
		const fnRefreshPhotos = () => this.refreshAlbum();
		const fnCloseConfirmDialog = () => this.setState({ confirmRemovePhotosOpen: false });

		upholiService.removePhotosFromAlbum(this.state.albumId, this.state.selectedPhotoIds)
			.then(() => {
				toast.info("Photos removed.");
				fnCloseConfirmDialog();
				fnRefreshPhotos();
			})
			.catch(console.error);
	}

	onFilesDropped(event: React.DragEvent<HTMLElement>): void {
		event.preventDefault();
		if (!event.dataTransfer.files || event.dataTransfer.files.length === 0)
			return; // no files

		this.uploadFilesList(event.dataTransfer.files);
	}

	uploadFilesList (fileList: FileList): void {
		const fnOnUploadFinished = () => {
			this.refreshAlbum();
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
		if (options.shared) {
			upholiService.upsertAlbumShare(this.state.albumId, options.password)
				.then(() => {
					this.refreshShare();
				})
				.catch(console.error);
		}
		else {
			if (this.state.share) {
				upholiService.deleteShare(this.state.share.id)
					.then(() => this.setState({ share: null }))
					.catch(console.error);
			}
		}
	}

	render(): React.ReactNode {
		if (!this.state.album) {
			return null;
		}
		else {
			return (
				<ContentContainer onDrop={(event) => this.onFilesDropped(event)}>
					<AlbumView
						album={this.state.album}
						selectedPhotos={this.state.selectedPhotoIds}
						onSelectionChanged={(selectedPhotoIds) => this.setState({selectedPhotoIds})}
						/>

					<ModalSharingOptions
						share={this.state.share}
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