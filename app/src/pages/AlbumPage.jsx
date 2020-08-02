import React from "react";
import PhotoGallerySelectable from "../components/PhotoGallerySelectable.jsx";
import PageLayout from "../components/PageLayout.jsx"
import AppStateContext from "../contexts/AppStateContext.jsx";
import PhotoService from "../services/PhotoService";
import UploadHelper from "../helpers/UploadHelper.js"
import ModalConfirmation from "../components/ModalConfirmation.jsx";
import ModalUploadProgress from "../components/ModalUploadProgress.jsx";
import ModalCopyUrl from "../components/ModalCopyUrl.jsx";
import UploadButton from "../components/UploadButton.jsx";
import { IconLink, IconUpload, IconRemove, IconImage } from "../components/Icons.jsx";
import { toast } from "react-toastify";
import Switch from "react-switch";

class AlbumPage extends React.Component {

	constructor(props) {
		super(props);

		this.state = {
			albumId: props.match.params.albumId,
			title: "",
			public: false,
			photos: [],
			selectedPhotos: [],
			copyPublicAlbumUrlModalOpen: false,
			confirmDeleteAlbumOpen: false,
			confirmRemovePhotosOpen: false,
			uploadInProgress: false,
			uploadFiles: []
		};
	}

	componentDidMount() {
		this.refreshPhotos();
	}

	refreshPhotos() {
		let _this = this;
		PhotoService.getAlbum(this.state.albumId)
			.then((response) => {
				_this.setState({
					title: response.title,
					public: response.public,
					photos: response.photos.map((photo) => {
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

	shareAlbum() {
		this.setState({
			copyPublicAlbumUrlModalOpen: true
		});
	}
	
	onDeleteAlbumClick() {
		this.setState({
			confirmDeleteAlbumOpen: true
		});
	}

	deleteAlbum() {
		let component = this;
		let albumTitle = this.state.title;

		PhotoService.deleteAlbum(this.state.albumId)
			.then(() => {
				toast.info("Album '" + albumTitle + "' deleted.");
				component.context.history.push("/albums");
			})
			.catch((error) => console.log(error));
	}

	onPhotoClicked(event, target) {
		let photo = this.state.photos[target.index];
		!!this.context.history && this.context.history.push("/photo/" + photo.id);
	}

	setSelectedPhotoAsAlbumCover() {
		let _refreshPhotos = () => this.refreshPhotos();
		
		let photoId = this.state.selectedPhotos[0];

		PhotoService.updateAlbumCover(this.state.albumId, photoId)
			.then(() => {
				toast.info("Album cover updated.");
				_refreshPhotos();
			})
			.catch(error => console.log(error));
	}

	onRemovePhotosClick() {
		this.setState({
			confirmRemovePhotosOpen: true
		});
	}

	removeSelectedPhotosFromAlbum() {
		let fnRefreshPhotos = () => this.refreshPhotos();
		let fnCloseConfirmDialog = () => this.setState({ confirmRemovePhotosOpen: false });
		
		let selectedPhotos = this.state.selectedPhotos;
		let photoIds = this.state.photos.map(p => p.id);
		let remainingPhotosAfterRemoval = photoIds.filter(id => selectedPhotos.indexOf(id) === -1);

		PhotoService.updateAlbumPhotos(this.state.albumId, remainingPhotosAfterRemoval)
			.then(() => {
				toast.info("Photos removed.");
				fnCloseConfirmDialog();
				fnRefreshPhotos();
			})
			.catch(error => console.log(error));
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

	onFilesDropped(event) {
		event.preventDefault();
		if (!event.dataTransfer.files || event.dataTransfer.files.length === 0)
			return; // no files

		this.uploadFilesList(event.dataTransfer.files);
	}

	uploadFilesList (filesList) {
		let albumId = this.state.albumId;
		let photoIds = this.state.photos.map(p => p.id)

		let files = UploadHelper.convertFileListToFileArrayForUploadDialog(filesList);
		
		let fnOnUploadFinished = (uploadedPhotoIds) => {
			this.setState({
				uploadInProgress: false,
				uploadFiles: []
			});

			let fnRefreshPhotos = () => this.refreshPhotos();

			if (uploadedPhotoIds && uploadedPhotoIds.length > 0) {
				toast.info("Upload finished.");

				photoIds = photoIds.concat(uploadedPhotoIds);
				PhotoService.updateAlbumPhotos(albumId, photoIds)
					.then(fnRefreshPhotos);
			}
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
		const headerActions = (<div>
			{this.state.selectedPhotos.length === 1 && <button className="iconOnly" onClick={(e) => this.setSelectedPhotoAsAlbumCover()} title="Set album cover">
				<IconImage/>
			</button>}
			{this.state.selectedPhotos.length > 0 && <button className="iconOnly" onClick={(e) => this.onRemovePhotosClick()} title="Remove from album">
				<IconRemove/>
			</button>}
			{this.state.selectedPhotos.length === 0 && <button className="iconOnly" onClick={() => document.getElementById("select-photos").click()} title="Upload photos">
				<IconUpload/>
			</button>}
		</div>);

		const headerContextMenuActions = (<div>
			{<button onClick={(e) => this.onDeleteAlbumClick()}>Delete album</button>}
		</div>);

		return (
			<PageLayout title={"Album - " + this.state.title} requiresAuthentication={true} headerActions={headerActions} headerContextMenuActions={headerContextMenuActions} onDrop={(event) => this.onFilesDropped(event)}>
				<div className="topBar">
					<h1>{this.state.title}</h1>

					{this.state.public && <button className="shareUrl iconOnly" onClick={() => this.setState({copyPublicAlbumUrlModalOpen: true})}>
						<IconLink/>
					</button>}

					<label className="switch">
						<Switch checked={this.state.public}
							width={80}
							onColor="#d3e532"
							checkedIcon={<span className="checkedIcon">Public</span>}
							uncheckedIcon={<span className="uncheckedIcon">Private</span>}
							onChange={(bPublic) => {
								PhotoService.updateAlbumPublic(this.state.albumId, bPublic);
								this.setState({
									public: bPublic
								});
							}}/>
					</label>
				</div>

				{!!this.state.title && this.state.photos.length === 0 && 
					<span className="centerText">This album has no photos.</span>
				}

				{this.state.photos.length > 0 && <PhotoGallerySelectable 
					onClick={(event, target) => this.onPhotoClicked(event, target)} 
					photos={this.state.photos} 
					selectedItems={this.state.selectedPhotos} 
					onPhotoSelectedChange={(photoId, selected) => this.onPhotoSelectedChange(photoId, selected)}/>
				}

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

				<ModalCopyUrl
					isOpen={this.state.copyPublicAlbumUrlModalOpen}
					onRequestClose={() => this.setState({copyPublicAlbumUrlModalOpen: false})}
					url={location.origin + "/shared/collection/" + this.state.albumId}
					/>

				<ModalUploadProgress
					isOpen={this.state.uploadInProgress}
					onRequestClose={() => this.setState({uploadInProgress: false})}
					files={this.state.uploadFiles}
					/>
					

				{/* Hidden upload button triggered by the button in action bar. This allos me to write simpler CSS to style the action buttons. */}
				<UploadButton className="hidden" onSubmit={(files) => this.uploadFilesList(files)}/>
			</PageLayout>
		);
	}
}

AlbumPage.contextType = AppStateContext;
export default AlbumPage;