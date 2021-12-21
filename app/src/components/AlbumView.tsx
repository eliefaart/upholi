import * as React from "react";
import appStateContext from "../contexts/AppStateContext";
import UrlHelper from "../helpers/UrlHelper";
import { Album } from "../models/Album";
import GalleryPhoto from "../models/GalleryPhoto";
import ModalPhotoDetail from "./modals/ModalPhotoDetail";
import PhotoGallery from "./gallery/PhotoGallery";
import usePhotoThumbnailSources from "../hooks/usePhotoThumbnailSources";
import { FC, useContext, useState } from "react";

const queryStringParamNamePhotoId = "photoId";

interface Props {
	album: Album,
	/** IDs of photos currently selected. */
	selectedPhotos?: string[],
	/** Called when selection changes. */
	onSelectionChanged?: (selectedPhotoIds: string[]) => void
}

const AlbumView: FC<Props> = (props: Props) => {
	const context = useContext(appStateContext);
	const photoSources = usePhotoThumbnailSources(props.album.photos);
	const [openedPhotoId, setOpenedPhotoId] = useState<string>("");

	// Open photo, if indicated as such by query string
	const queryStringPhotoId = UrlHelper.getQueryStringParamValue(location.search, queryStringParamNamePhotoId);
	if (openedPhotoId !== queryStringPhotoId) {
		setOpenedPhotoId(queryStringPhotoId);
	}

	const onPhotoClicked = (photoId: string): void => {
		if (photoId) {
			context.history.push(document.location.pathname + "?photoId=" + photoId);
		}
	};

	const galleryPhotos = props.album.photos.map((photo): GalleryPhoto => {
		return {
			id: photo.id,
			src: photoSources.find(ps => ps.photoId === photo.id)?.src ?? "",
			width: photo.width,
			height: photo.height
		};
	});

	return <div className="album-view">
		<div className="top-bar">
			<h1>{props.album.title}</h1>
		</div>

		{!!props.album.title && galleryPhotos.length === 0 &&
			<span className="center-text">This album has no photos.</span>
		}

		{galleryPhotos.length > 0 && <PhotoGallery
			onClick={onPhotoClicked}
			photos={galleryPhotos}
			selectedItems={props.selectedPhotos ?? []}
			onPhotoSelectionChanged={props.onSelectionChanged} />
		}

		{openedPhotoId && <ModalPhotoDetail
			isOpen={!!openedPhotoId}
			photoId={openedPhotoId}
			photoKey={props.album.photos.find(p => p.id === openedPhotoId)?.key ?? undefined}
			onRequestClose={() => context.history.push(document.location.pathname + "?" + UrlHelper.removeQueryStringParam(document.location.search, queryStringParamNamePhotoId))}
		/>}
	</div>;
};

export default AlbumView;
