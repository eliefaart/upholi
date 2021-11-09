import * as React from "react";
import appStateContext from "../contexts/AppStateContext";
import UrlHelper from "../helpers/UrlHelper";
import { Album } from "../models/Album";
import GalleryPhoto from "../models/GalleryPhoto";
import upholiService from "../services/UpholiService";
import ModalPhotoDetail from "./modals/ModalPhotoDetail";
import PhotoGallery from "./misc/PhotoGallery";

const queryStringParamNamePhotoId = "photoId";

interface Props {
	album: Album,
	/** IDs of photos currently selected. */
	selectedPhotos?: string[],
	/** Called when selection changes. */
	onSelectionChanged?: (selectedPhotoIds: string[]) => void
}

interface State {
	photoSources: StatePhotoSource[],
	openedPhotoId: string
}

interface StatePhotoSource {
	photoId: string,
	src: string
}

export default class AlbumView extends React.Component<Props, State> {
	static contextType = appStateContext;

	constructor(props: Props) {
		super(props);

		this.state = {
			photoSources: [],
			openedPhotoId: ""
		};
	}

	componentDidMount(): void {
		this.fetchPhotoSources();
	}

	componentDidUpdate(): void {
		// Open photo, if indicated as such by query string
		const queryStringPhotoId = UrlHelper.getQueryStringParamValue(location.search, queryStringParamNamePhotoId);
		if (this.state.openedPhotoId !== queryStringPhotoId) {
			this.setState(() => {
				return {
					openedPhotoId: queryStringPhotoId
				};
			});
		}

		this.fetchPhotoSources();
	}

	fetchPhotoSources(): void {
		const photoSourcesMissing: StatePhotoSource[] = this.props.album.photos
			.filter(photo => !this.state.photoSources.some(ps => ps.photoId === photo.id))
			.map(photo => {
				return {
					photoId: photo.id,
					src: ""
				};
			});

		// Update state with prepared photoSource objects
		if (photoSourcesMissing.length > 0) {
			this.setState(previousState => {
				const photoSources = previousState.photoSources;
				for (const photoSource of photoSourcesMissing) {
					photoSources.push(photoSource);
				}

				return {
					photoSources
				};
			});

			// Fetch each missing one
			for (const photo of photoSourcesMissing) {
				const albumPhoto = this.props.album.photos.find(p => p.id === photo.photoId);
				upholiService.getPhotoThumbnailImageSrc(photo.photoId, albumPhoto?.key ?? undefined)
					.then(src => {
						this.setState(previousState => {
							const photoSourceToUpdate = previousState.photoSources.find(p => p.photoId === photo.photoId);
							if (photoSourceToUpdate) {
								photoSourceToUpdate.src = src;
							}

							return {
								photoSources: previousState.photoSources
							};
						});
					});
			}
		}
	}

	onPhotoClicked(photoIndex: number): void {
		const photo = this.props.album.photos[photoIndex];
		this.context.history.push(document.location.pathname + "?photoId=" + photo.id);
	}

	// onPhotoSelectedChange(photoId: string, selected: boolean): void {
	// 	if (this.props.onSelectionChanged) {
	// 		const selectedPhotos = this.props.selectedPhotos ?? [];

	// 		if (selected) {
	// 			selectedPhotos.push(photoId);
	// 		} else {
	// 			const index = selectedPhotos.indexOf(photoId);
	// 			if (index > -1) {
	// 				selectedPhotos.splice(index, 1);
	// 			}
	// 		}

	// 		this.props.onSelectionChanged(selectedPhotos);
	// 	}
	// }

	render(): React.ReactNode {
		const galleryPhotos = this.props.album.photos.map((photo): GalleryPhoto => {
			return {
				id: photo.id,
				src: this.state.photoSources.find(ps => ps.photoId === photo.id)?.src ?? "",
				width: photo.width,
				height: photo.height
			};
		});

		return <div className="album-view">
			<div className="topBar">
				<h1>{this.props.album.title}</h1>
			</div>

			{!!this.props.album.title && galleryPhotos.length === 0 &&
				<span className="centerText">This album has no photos.</span>
			}

			{galleryPhotos.length > 0 && <PhotoGallery
				onClick={(_, target) => this.onPhotoClicked(target.index)}
				photos={galleryPhotos}
				selectedItems={this.props.selectedPhotos ?? []}
				onPhotoSelectionChanged={this.props.onSelectionChanged}/>
			}

			{this.state.openedPhotoId && <ModalPhotoDetail
				isOpen={!!this.state.openedPhotoId}
				photoId={this.state.openedPhotoId}
				photoKey={this.props.album.photos.find(p => p.id === this.state.openedPhotoId)?.key ?? undefined}
				onRequestClose={() => this.context.history.push(document.location.pathname + "?" + UrlHelper.removeQueryStringParam(document.location.search, queryStringParamNamePhotoId))}
			/>}
		</div>;
	}
}