import React from "react";
import PhotoGallerySelectable from "../components/PhotoGallerySelectable.jsx";
import AppStateContext from "../contexts/AppStateContext.jsx";
import PhotoService from "../services/PhotoService";
import Albums from "../components/Albums.jsx";
import ModalPhotoDetail from "../components/ModalPhotoDetail.jsx";
import UrlHelper from "../helpers/UrlHelper.ts";

const queryStringParamNameAlbumId = "albumId";
const queryStringParamNamePhotoId = "photoId";

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
		let initialActiveAlbumId = UrlHelper.getQueryStringParamValue(location.search, queryStringParamNameAlbumId);

		// If there is only one album in collection, open it by default
		if (!initialActiveAlbumId && this.collectionHasOneAlbum) {
			initialActiveAlbumId = this.state.collection.albums[0].id;
		}

		if (initialActiveAlbumId){
			this.loadAlbum(initialActiveAlbumId);
		}
	}

	componentDidUpdate() {
		// Open photo, if indicated as such by query string
		let queryStringPhotoId = UrlHelper.getQueryStringParamValue(location.search, queryStringParamNamePhotoId);
		if (this.state.openedPhotoId !== queryStringPhotoId) {
			this.setState({
				openedPhotoId: queryStringPhotoId
			});
		}
	}

	openAlbum(albumId) {
		this.loadAlbum(albumId);
		this.setLocationPath(albumId);
	}

	loadAlbum(albumId) {
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

	/**
	 * Update the current browser location to match the currently opened album.
	 */
	setLocationPath(albumId) {
		const initialQueryString = location.search;
		const newQueryString = UrlHelper.setQueryStringParam(location.search, queryStringParamNameAlbumId, albumId);

		if (initialQueryString !== newQueryString) {
			let newUrl = location.pathname + "?" + newQueryString;
			this.context.history.replace(newUrl);
		}
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
		const photoIdUrl = document.location.pathname + "?" + UrlHelper.setQueryStringParam(document.location.search, queryStringParamNamePhotoId, photo.id);
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
					onRequestClose={() => this.context.history.push(document.location.pathname + "?" + UrlHelper.removeQueryStringParam(document.location.search, queryStringParamNamePhotoId))}
					/>
			</div>
		);
	}
}

CollectionView.contextType = AppStateContext;
export default CollectionView;