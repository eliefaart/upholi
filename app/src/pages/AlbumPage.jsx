import React from 'react';
import PhotoGallerySelectable from '../components/PhotoGallerySelectable.jsx';
import PhotoDetail from '../components/PhotoDetail.jsx';
import PageLayout from "../components/PageLayout.jsx"

import AppStateContext from '../contexts/AppStateContext.jsx';
import PhotoService from '../services/PhotoService';
import Overlay from '../components/Overlay.jsx';
import ConfirmationDialog from '../components/ConfirmationDialog.jsx';

class AlbumPage extends React.Component {

	constructor(props) {
		super(props);

		this.state = {
			albumId: props.match.params.albumId,
			title: "",
			photos: [],
			selectedPhotos: [],
			confirmDeleteAlbumOpen: false,
			confirmRemovePhotosOpen: false,
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
		PhotoService.getAlbum(this.state.albumId)
			.then((response) => {
				_this.setState({
					title: response.title,
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
		// Open dialog, dialog contains checkbox/toggle button.
		// On enabled:
		//  - Generate a public link to album.
		// On disabled:
		//	- Delete generated public link.
	}
	
	onDeleteAlbumClick() {
		this.setState({
			confirmDeleteAlbumOpen: true
		});
	}

	deleteAlbum() {
		PhotoService.deleteAlbum(this.state.albumId)
			.then(() => this.context.history.push("/albums"))
			.catch((error) => console.log(error));
	}

	onPhotoClicked(event, target) {
		let photo = this.state.photos[target.index];

		!!this.context.history && this.context.history.push("/photo/" + photo.id);
	}

	closePhoto() {
		this.setState({
			openedPhotoId: null
		});
	}

	setSelectedPhotoAsAlbumCover() {
		let _refreshPhotos = () => this.refreshPhotos();
		
		let photoId = this.state.selectedPhotos[0];

		PhotoService.updateAlbumCover(this.state.albumId, photoId)
			.then(() => _refreshPhotos())
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

		let albumId = this.state.albumId;
		let photoIds = this.state.photos.map(p => p.id)
		let fnRefreshPhotos = () => this.refreshPhotos();

		PhotoService.uploadPhotos(event.dataTransfer.files)
			.then((values) => {
				if (values && values.length > 0) {
					photoIds = photoIds.concat(values);
					PhotoService.updateAlbumPhotos(albumId, photoIds)
						.then(() => {
							fnRefreshPhotos();
						});
				}
			});
	}

	render() {
		const headerActions = (<div>
			{this.state.selectedPhotos.length === 1 && <button onClick={(e) => this.setSelectedPhotoAsAlbumCover()}>Set as cover</button>}
			{this.state.selectedPhotos.length > 0 && <button onClick={(e) => this.onRemovePhotosClick()}>Remove photos</button>}
			{<button onClick={(e) => this.shareAlbum()}>Share</button>}
			{<button onClick={(e) => this.onDeleteAlbumClick()}>Delete album</button>}
		</div>);
		
		return (
			<PageLayout headerElementActions={headerActions} onDrop={(event) => this.onFilesDropped(event)}>
				<PhotoGallerySelectable onClick={(event, target) => this.onPhotoClicked(event, target)} photos={this.state.photos} selectedItems={this.state.selectedPhotos} onPhotoSelectedChange={(photoId, selected) => this.onPhotoSelectedChange(photoId, selected)}/>

				{!!this.state.openedPhotoId && 
					<Overlay onClick={() => this.closePhoto()}>
						{!!this.state.openedPhotoId  && <PhotoDetail src={PhotoService.baseUrl() + "/photo/" + this.state.openedPhotoId + "/original"}/>}
					</Overlay>
				}

				<ConfirmationDialog
					title="Delete?"
					isOpen={this.state.confirmDeleteAlbumOpen}
					onRequestClose={() => this.setState({confirmDeleteAlbumOpen: false})}
					onOkButtonClick={() => this.deleteAlbum()}
					okButtonText="Delete"
					confirmationText={"Album '" + this.state.title + "' will be deleted."}
					/>
				<ConfirmationDialog
					title="Delete?"
					isOpen={this.state.confirmRemovePhotosOpen}
					onRequestClose={() => this.setState({confirmRemovePhotosOpen: false})}
					onOkButtonClick={() => this.removeSelectedPhotosFromAlbum()}
					okButtonText="Delete"
					confirmationText={this.state.selectedPhotos.length + " photos will be removed from album '" + this.state.title + "'."}
					/>
			</PageLayout>
		);
	}
}

AlbumPage.contextType = AppStateContext;
export default AlbumPage;