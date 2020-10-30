import React from "react";
import PhotoGallerySelectable from "../components/PhotoGallerySelectable.jsx";
import AppStateContext from "../contexts/AppStateContext.jsx";
import PhotoService from "../services/PhotoService";
import Albums from "../components/Albums.jsx";

class CollectionView extends React.Component {

	constructor(props) {
		super(props);

		this.state = {
			collection: props.collection,
			activeAlbum: {
				id: null,
				title: null,
				photos: []
			},
		};
	}

	componentDidMount() {
		let defaultActiveAlbumId = this.props.initialActiveAlbumId;

		// If there is only one album in collection, open it by default
		if (!defaultActiveAlbumId && this.state.collection.albums.length === 1) {
			defaultActiveAlbumId = this.state.collection.albums[0].id;
		}

		if (defaultActiveAlbumId){
			this.openAlbum(defaultActiveAlbumId);
		}
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

		this.setLocationPath(albumId);
	}

	/**
	 * Update the current browser location to match the currently opened album.
	 */
	setLocationPath(albumId) {
		const locationPathAlbumPartIndex = location.pathname.indexOf("/album/");
		const collectionBaseUrl = locationPathAlbumPartIndex !== -1
			? location.pathname.substr(0, locationPathAlbumPartIndex)
			: location.pathname;

		if (collectionBaseUrl.endsWith("/")) {
			collectionBaseUrl = collectionBaseUrl.substr(0, collectionBaseUrl.length - 1);
		}

		let newUrl = collectionBaseUrl;
		if (albumId) {
			newUrl += "/album/" + albumId;
		}

		this.context.history.replace(newUrl);
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

			this.setLocationPath(null);
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