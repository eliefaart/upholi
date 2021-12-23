import * as React from "react";
import { FC } from "react";
import { default as Gallery, PhotoProps } from "react-photo-gallery";
import { default as GalleryPhotoModel } from "../../models/GalleryPhoto";
import GalleryPhoto from "./GalleryPhoto";

/** Target height for algorithm, but exact height will vary a bit. */
const PHOTO_HEIGHT = 200;
const PHOTO_MARGIN = 3;

interface Props {
	photos: GalleryPhotoModel[],
	selectedItems: string[],
	onPhotoSelectionChanged?: (photoIds: string[]) => void,
	onClick: (photoId: string) => void
}

function getGalleryViewModel(photos: GalleryPhotoModel[]): PhotoProps[] {
	return photos.map<PhotoProps>(photo => {
		return {
			key: photo.id,
			src: photo.src,
			width: photo.width,
			height: photo.height
		};
	});
}

const PhotoGallery: FC<Props> = (props) => {
	const galleryViewModel = getGalleryViewModel(props.photos);
	const anyPhotoSelected = props.selectedItems.length > 0;

	return (
		<div className="photo-gallery">
			<Gallery photos={galleryViewModel}
				margin={PHOTO_MARGIN}
				targetRowHeight={PHOTO_HEIGHT}
				renderImage={(renderProps) => {
					const photoId = renderProps.photo.key ?? "";
					const selected = props.selectedItems.indexOf(photoId) !== -1;

					// Toggle selected state of current photo
					const fnToggleSelected = () => {
						if (props.onPhotoSelectionChanged) {
							// If selected, then on callback we must unselect. And visa versa
							const newSelectedPhotoIds = selected
								? props.selectedItems.filter(pId => pId !== photoId)
								: [photoId, ...props.selectedItems];

							props.onPhotoSelectionChanged(newSelectedPhotoIds);
						}
					};

					const gpiProps = {
						...renderProps,
						selected,
						anySiblingPhotoSelected: props.selectedItems.length > 0,
						onClick: () => {
							if (anyPhotoSelected) {
								fnToggleSelected();
							}
							else {
								props.onClick(photoId);
							}

						},
						onToggleSelect: fnToggleSelected,
					};
					return GalleryPhoto(gpiProps);
				}}
			/>
		</div>
	);
};

export default PhotoGallery;