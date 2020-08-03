import React from "react";
import PhotoGallerySelectable from "../components/PhotoGallerySelectable.jsx";
import PageLayout from "../components/PageLayout.jsx";
import PhotoService from "../services/PhotoService.js";
import UploadHelper from "../helpers/UploadHelper.js";
import AppStateContext from "../contexts/AppStateContext.jsx";
import ModalConfirmation from "../components/ModalConfirmation.jsx";
import ModalUploadProgress from "../components/ModalUploadProgress.jsx";
import ModalCreateAlbum from "../components/ModalCreateAlbum.jsx";
import ModalAddToAlbum from "../components/ModalAddToAlbum.jsx";
import UploadButton from "../components/UploadButton.jsx";
import { IconUpload, IconDelete, IconAddToAlbum } from "../components/Icons.jsx";
import { toast } from "react-toastify";

class PhotosDashboardPage extends React.Component {

	constructor(props) {
		super(props);

		this.state = {
			photos: [],
			selectedPhotos: [],
			newAlbumDialogOpen: false,
			confirmDeletePhotosOpen: false,
			addPhotosToAlbumDialogOpen: false,
			albums: [],
			uploadInProgress: false,
			uploadFiles: []
		};
	}

	componentDidMount() {
		this.refreshPhotos();
	}

	refreshPhotos() {
		const fnSetPhotos = (photos) => this.setState({ photos });

		PhotoService.getPhotos()
			.then(fnSetPhotos)
			.catch(console.error);
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
		!!this.context.history && this.context.history.push("/photo/" + photo.id);
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
		const headerActions = (<div>
			{this.state.selectedPhotos.length === 0 && <button className="iconOnly" onClick={() => document.getElementById("select-photos").click()} title="Upload photos">
				<IconUpload/>
			</button>}
			{this.state.selectedPhotos.length > 0 && <button className="iconOnly" onClick={() => this.onClickAddSelectedPhotosToAlbum(this.state.selectedPhotos)} title="Add to album">
				<IconAddToAlbum/>
			</button>}
			{this.state.selectedPhotos.length > 0 && <button className="iconOnly" onClick={() => this.onClickDeletePhotos()} title="Delete photos">
				<IconDelete/>
			</button>}
		</div>);

		return (
			<PageLayout title="Library" requiresAuthentication={true} headerActions={headerActions} onDrop={(event) => this.onFilesDropped(event)}>
				<PhotoGallerySelectable photos={this.state.photos} onClick={(event, target) => this.onPhotoClicked(event, target)} selectedItems={this.state.selectedPhotos} onPhotoSelectedChange={(photoId, selected) => this.onPhotoSelectedChange(photoId, selected)} />

				<ModalCreateAlbum
					isOpen={this.state.newAlbumDialogOpen}
					onRequestClose={() => this.setState({newAlbumDialogOpen: false})}
					createWithPhotos={this.state.selectedPhotos}
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
			</PageLayout>
		);
	}
}

PhotosDashboardPage.contextType = AppStateContext;
export default PhotosDashboardPage;