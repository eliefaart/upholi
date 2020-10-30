import React from "react";
import PhotoGallerySelectable from "../components/PhotoGallerySelectable.jsx";
import AppStateContext from "../contexts/AppStateContext.jsx";
import PhotoService from "../services/PhotoService";
import Albums from "../components/Albums.jsx";

class CollectionView extends React.Component {

	constructor(props) {
		super(props);

		this.state = {
			collection: this.props.collection,
			activeAlbum: {
				id: null,
				title: null,
				photos: []
			},
		};
	}

	openAlbum(albumId) {
		let _this = this;
		PhotoService.getAlbum(albumId)
			.then((response) => {
				_this.setState({
					activeAlbum: {
						id: albumId,
						title: response.title,
						photos: response.photos.map((photo) => {
							return {
								id: photo.id,
								src: PhotoService.baseUrl() + "/photo/" + photo.id + "/thumb",
								width: photo.width,
								height: photo.height
							};
						})
					}
				});
			});
	}

	onAlbumClicked(album) {
		if (album.id === this.state.activeAlbum.id) {
			this.setState({
				activeAlbum: {
					id: null,
					title: null,
					photos: []
				}
			});
		}
		else {
			this.openAlbum(album.id);
		}
	}

	onPhotoClicked(_, target) {
		const photo = this.state.activeAlbum.photos[target.index];
		!!this.context.history && this.context.history.push("/photo/" + photo.id);
	}

	render() {
		if (this.state.collection == null)
			return null;

		return (
			<div className="collection">
				<div className="topBar">
					<h1>{this.state.collection.title}</h1>
				</div>

				{/* Albums in this collection */}
				<Albums
					albums={this.state.collection.albums}
					activeAlbumId={this.state.activeAlbum.id}
					onClick={album => this.onAlbumClicked(album)}/>

				{/* Photos inside selected/active album */}
				{!!this.state.activeAlbum.id && <div className="photos">
					<h2>{this.state.activeAlbum.title}</h2>
					{this.state.activeAlbum.photos.length > 0 && <PhotoGallerySelectable
						onClick={(event, target) => this.onPhotoClicked(event, target)}
						photos={this.state.activeAlbum.photos}
						selectedItems={[]}
						onPhotoSelectedChange={() => {}}/>
					}
				</div>}
			</div>
		);
	}
}

CollectionView.contextType = AppStateContext;
export default CollectionView;