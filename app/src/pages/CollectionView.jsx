import React from "react";
import queryString from "query-string";
import PhotoGallerySelectable from "../components/PhotoGallerySelectable.jsx";
import AppStateContext from "../contexts/AppStateContext.jsx";
import PhotoService from "../services/PhotoService";
import Albums from "../components/Albums.jsx";
import ModalPhotoDetail from "../components/ModalPhotoDetail.jsx";
import UrlHelper from "../helpers/UrlHelper.js";

class CollectionView extends React.Component {

	constructor(props) {
		super(props);

		this.collectionHasOneAlbum = props.collection.albums.length === 1;

		this.state = {
			collection: props.collection,
			activeAlbum: {
				id: null,
				title: null,
				photos: []
			},
			openedPhotoId: null,
		};
	}

	componentDidMount() {
		let defaultActiveAlbumId = null;

		// Parse from query string
		const queryStringParams = queryString.parse(location.search);
		if (queryStringParams["album"]) {
			defaultActiveAlbumId = queryStringParams.album;
		}

		// If there is only one album in collection, open it by default
		if (!defaultActiveAlbumId && this.collectionHasOneAlbum) {
			defaultActiveAlbumId = this.state.collection.albums[0].id;
		}

		if (defaultActiveAlbumId){
			this.openAlbum(defaultActiveAlbumId);
		}
	}

	componentDidUpdate() {
		// Open photo, if indicated as such by query string
		const queryStringParams = queryString.parse(location.search);
		queryStringParams.photoId = queryStringParams.photoId || null;
		if (this.state.openedPhotoId !== queryStringParams.photoId) {
			this.setState({
				openedPhotoId: queryStringParams.photoId
			});
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
		let newUrl = location.pathname;
		if (albumId) {
			newUrl += "?album=" + albumId;
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
		const photoIdUrl = document.location.pathname + "?" + UrlHelper.addQueryStringParam(document.location.search, "photoId", photo.id);
		this.context.history.push(photoIdUrl);
	}

	render() {
		if (this.state.collection == null)
			return null;

		return (
			<div className="collection">
				<div className="topBar">
					<h1>{this.collectionHasOneAlbum ? this.state.activeAlbum.title : this.state.collection.title}</h1>
				</div>

				{/* Albums in this collection */}
				{!this.collectionHasOneAlbum && <Albums
					albums={this.state.collection.albums}
					activeAlbumId={this.state.activeAlbum.id}
					onClick={album => this.onAlbumClicked(album)}/>
				}

				{/* Photos inside selected/active album */}
				{!!this.state.activeAlbum.id && <div className="photos">
					{!this.collectionHasOneAlbum && <h2>{this.state.activeAlbum.title}</h2>}
					{this.state.activeAlbum.photos.length > 0 && <PhotoGallerySelectable
						onClick={(event, target) => this.onPhotoClicked(event, target)}
						photos={this.state.activeAlbum.photos}
						selectedItems={[]}
						onPhotoSelectedChange={() => {}}/>
					}
				</div>}

				<ModalPhotoDetail
					isOpen={!!this.state.openedPhotoId}
					photoId={this.state.openedPhotoId}
					onRequestClose={() => this.context.history.push(document.location.pathname + "?" + UrlHelper.removeQueryStringParam(document.location.search, "photoId"))}
					/>
			</div>
		);
	}
}

CollectionView.contextType = AppStateContext;
export default CollectionView;