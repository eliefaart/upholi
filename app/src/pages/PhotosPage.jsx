import React from 'react';
import Modal from '../components/Modal.jsx';
import PhotoGallerySelectable from '../components/PhotoGallerySelectable.jsx';
import PageLayout from "../components/PageLayout.jsx";
import Albums from "../components/Albums.jsx";
import PhotoService from "../services/PhotoService.js";
import UploadHelper from "../helpers/UploadHelper.js";
import AppStateContext from '../contexts/AppStateContext.jsx';
import ConfirmationDialog from '../components/ConfirmationDialog.jsx';
import UploadProgressDialog from '../components/UploadProgressDialog.jsx';
import ModalCreateAlbum from '../components/ModalCreateAlbum.jsx';
import UploadButton from '../components/UploadButton.jsx';
import { toast } from 'react-toastify';
import $ from 'jquery';

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

	componentWillUnmount() {
	}

	refreshPhotos() {
		let _this = this;
		PhotoService.getPhotos((photos) => {
			_this.setState({
				photos: photos
			});
		});
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
		PhotoService.deletePhotos(this.state.selectedPhotos, () => {
			let remainingPhotos = this.state.photos.filter(p => 
				this.state.selectedPhotos.indexOf(p.id) === -1);

			this.setState({
				photos: remainingPhotos,
				selectedPhotos: [],
				confirmDeletePhotosOpen: false
			});

			toast.info("Photos deleted.");
		});
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
		const headerContextMenuActions = (<div>
			{this.state.selectedPhotos.length === 0 && <button onClick={() => $("#select-photos").click()}>Upload</button>}
			{this.state.selectedPhotos.length > 0 && <button onClick={() => this.onClickAddSelectedPhotosToAlbum(this.state.selectedPhotos)}>Add to album</button>}
			{this.state.selectedPhotos.length > 0 && <button onClick={() => this.onClickDeletePhotos()}>Delete</button>}
		</div>);

		return (
			<PageLayout headerContextMenuActions={headerContextMenuActions} onDrop={(event) => this.onFilesDropped(event)}>
				<PhotoGallerySelectable photos={this.state.photos} onClick={(event, target) => this.onPhotoClicked(event, target)} selectedItems={this.state.selectedPhotos} onPhotoSelectedChange={(photoId, selected) => this.onPhotoSelectedChange(photoId, selected)} />

				{this.state.newAlbumDialogOpen && <ModalCreateAlbum
					isOpen={this.state.newAlbumDialogOpen}
					onRequestClose={() => this.setState({newAlbumDialogOpen: false})}
					createWithPhotos={this.state.selectedPhotos}
					/>}

				{this.state.addPhotosToAlbumDialogOpen && 
					<Modal
						title="Choose album"
						isOpen={this.state.addPhotosToAlbumDialogOpen}
						onRequestClose={() => this.setState({addPhotosToAlbumDialogOpen: false})}
						onOkButtonClick={null}
						okButtonText={null}
						>
							<button onClick={() => this.createNewAlbum()}>New album</button>
							<Albums onClick={(album) => this.addSelectedPhotosToAlbum(album.id)}/>
					</Modal>}

				{this.state.confirmDeletePhotosOpen && 
					<ConfirmationDialog
						title="Delete?"
						isOpen={this.state.confirmDeletePhotosOpen}
						onRequestClose={() => this.setState({confirmDeletePhotosOpen: false})}
						onOkButtonClick={() => this.deleteSelectedPhotos(this.state.selectedPhotos)}
						okButtonText="Delete"
						confirmationText={this.state.selectedPhotos.length + " photos will be deleted."}
						/>}

				{this.state.uploadInProgress && 
					<UploadProgressDialog
						isOpen={this.state.uploadInProgress}
						onRequestClose={() => this.setState({uploadInProgress: false})}
						files={this.state.uploadFiles}
						/>
					}
				{/* Hidden upload button triggered by the button context menu. This is because browsers prevent file select functionality if input element is no visible */}
				<UploadButton className="hidden" onSubmit={(files) => this.uploadFilesList(files)}/>
			</PageLayout>
		);
	}
}

PhotosDashboardPage.contextType = AppStateContext;
export default PhotosDashboardPage;