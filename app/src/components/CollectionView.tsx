import * as React from "react";
import PhotoGallerySelectable from "../components/PhotoGallerySelectable";
import appStateContext from "../contexts/AppStateContext";
import PhotoService from "../services/PhotoService";
import ModalPhotoDetail from "./modals/ModalPhotoDetail";
import UrlHelper from "../helpers/UrlHelper";
import Collection from "../models/Collection";
import GalleryPhoto from "../models/GalleryPhoto";
import Menu from "./Menu";
import upholiService from "../services/UpholiService";

const queryStringParamNameAlbumId = "albumId";
const queryStringParamNamePhotoId = "photoId";

interface CollectionViewProps {
	collection: Collection
}

interface CollectionViewState {
	collection: Collection,
	activeAlbum: ActiveAlbum | null,
	openedPhotoId: string | null
}

interface ActiveAlbum {
	id: string,
	title: string,
	photos: GalleryPhoto[]
}

class CollectionView extends React.Component<CollectionViewProps, CollectionViewState> {

	collectionHasOneAlbum: boolean;

	constructor(props:CollectionViewProps) {
		super(props);

		this.collectionHasOneAlbum = props.collection.albums.length === 1;

		this.state = {
			collection: props.collection,
			activeAlbum: null,
			openedPhotoId: null,
		};
	}

	componentDidMount(): void {
		let initialActiveAlbumId = UrlHelper.getQueryStringParamValue(location.search, queryStringParamNameAlbumId);

		// If there is only one album in collection, open it by default
		if (!initialActiveAlbumId && this.collectionHasOneAlbum) {
			initialActiveAlbumId = this.state.collection.albums[0].id;
		}

		if (initialActiveAlbumId){
			this.loadAlbum(initialActiveAlbumId);
		}
	}

	componentDidUpdate(): void {
		// Open photo, if indicated as such by query string
		const queryStringPhotoId = UrlHelper.getQueryStringParamValue(location.search, queryStringParamNamePhotoId);
		if (this.state.openedPhotoId !== queryStringPhotoId) {
			this.setState({
				openedPhotoId: queryStringPhotoId
			});
		}
	}

	openAlbum(albumId: string): void {
		if (albumId !== this.state.activeAlbum?.id) {
			this.loadAlbum(albumId);
			this.setLocationPath(albumId);
		}
	}

	loadAlbum(albumId:string): void {
		upholiService.getAlbum(albumId)
			.then((response) => {
				this.setState({
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
	setLocationPath(albumId: string): void {
		const initialQueryString = location.search;
		const newQueryString = UrlHelper.setQueryStringParam(location.search, queryStringParamNameAlbumId, albumId);

		if (initialQueryString !== newQueryString) {
			const newUrl = location.pathname + "?" + newQueryString;
			this.context.history.replace(newUrl);
		}
	}

	onPhotoClicked(index: number): void {
		if (this.state.activeAlbum) {
			const photo = this.state.activeAlbum.photos[index];
			const photoIdUrl = document.location.pathname + "?" + UrlHelper.setQueryStringParam(document.location.search, queryStringParamNamePhotoId, photo.id);
			this.context.history.push(photoIdUrl);
		}
	}

	render(): React.ReactNode {
		if (this.state.collection == null)
			return null;

		return (
			<div className="collection-view">
				{this.state.collection.albums.length > 1 && <Menu items={this.state.collection.albums.map(album => {
					return {
						title: album.title,
						active: album.id === this.state.activeAlbum?.id,
						onClick: () => this.openAlbum(album.id)
					};
				})}/>}

				{/* Photos inside selected/active album */}
				{!!this.state.activeAlbum && <div className="photos">
					<div className="topBar">
						<h1>{this.state.activeAlbum.title}</h1>
					</div>

					{this.state.activeAlbum.photos.length > 0 && <PhotoGallerySelectable
						onClick={(_, target) => this.onPhotoClicked(target.index)}
						photos={this.state.activeAlbum.photos}
						selectedItems={[]}/>
					}
				</div>}

				{this.state.openedPhotoId && <ModalPhotoDetail
					isOpen={!!this.state.openedPhotoId}
					photoId={this.state.openedPhotoId}
					onRequestClose={() => this.context.history.push(document.location.pathname + "?" + UrlHelper.removeQueryStringParam(document.location.search, queryStringParamNamePhotoId))}
				/>}
			</div>
		);
	}
}

CollectionView.contextType = appStateContext;
export default CollectionView;