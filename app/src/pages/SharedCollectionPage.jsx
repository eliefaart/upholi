import React from "react";
import PhotoGallerySelectable from "../components/PhotoGallerySelectable.jsx";
import PageLayout from "../components/PageLayout.jsx"
import AppStateContext from "../contexts/AppStateContext.jsx";
import PhotoService from "../services/PhotoService";

class SharedCollectionPage extends React.Component {

	constructor(props) {
		super(props);

		this.state = {
			collectionId: props.match.params.collectionId,
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

	refreshPhotos() {
		let _this = this;
		PhotoService.getAlbum(this.state.collectionId)
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

	onPhotoClicked(event, target) {
		let photo = this.state.photos[target.index];
		!!this.context.history && this.context.history.push("/photo/" + photo.id);
	}

	render() {
		return (
			<PageLayout requiresAuthentication={false} onDrop={(event) => this.onFilesDropped(event)} renderMenu={false}>
				<div className="topBar">
					<h1>{this.state.title}</h1>
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
			</PageLayout>
		);
	}
}

SharedCollectionPage.contextType = AppStateContext;
export default SharedCollectionPage;