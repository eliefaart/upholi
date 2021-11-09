import * as React from "react";
import { FC } from "react";
import { default as Gallery, RenderImageProps, PhotoProps} from "react-photo-gallery";
import GalleryPhoto from "../../models/GalleryPhoto";

/** Target height for algorithm, but exact height will vary a bit. */
const PHOTO_HEIGHT = 200;
const PHOTO_MARGIN = 3;

interface Props {
	photos: GalleryPhoto[],
	selectedItems: string[],
	onPhotoSelectionChanged?: (photoIds: string[]) => void,
	onClick: (event: React.MouseEvent<Element, MouseEvent>, photo: { index: number }) => void
}

function getGalleryViewModel(photos: GalleryPhoto[]): PhotoProps[] {
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
	const photosSelectable = props.onPhotoSelectionChanged !== undefined;
	const galleryViewModel = getGalleryViewModel(props.photos);

	// Inline-component representing one photo tile.
	//{ index: number, onClick, photo: Photo, margin: number, direction: string, top: number, left: number, key: string }
	const imageRenderer: FC<RenderImageProps<Record<string, never>>> = (renderImageProps: RenderImageProps<Record<string, never>>) => {
		if (renderImageProps.photo.key) {
			const photoId = renderImageProps.photo.key;
			const imgStyle: React.CSSProperties = {
				backgroundImage: "url(\"" + renderImageProps.photo.src + "\")",
				margin: renderImageProps.margin,
				width: renderImageProps.photo.width,
				height: renderImageProps.photo.height,
			};

			if (renderImageProps.direction === "column") {
				imgStyle.position = "absolute";
				imgStyle.left = renderImageProps.left;
				imgStyle.top = renderImageProps.top;
			}

			const containerStyle: React.CSSProperties = {
				position: "relative"
			};

			const photoMargin = parseInt(renderImageProps.margin ?? "0");
			const checkboxLabelStyle: React.CSSProperties = {
				position: "absolute",
				top: 0 + (photoMargin * 2),
				left: 0 + (photoMargin * 2),
			};

			const checkboxId = "photo_select_" + photoId;
			const isSelected = props.selectedItems.indexOf(photoId) !== -1;
			const anySelected = props.selectedItems.length > 0;
			const cssClass = "photo"
				+ " " + (photosSelectable ? "selectable" : "")
				+ " " + (isSelected ? "selected" : "")
				+ " " + (anySelected ? "any-other-selected" : "");

			const changePhotoSelectedState = (selected: boolean): void => {
				console.log("changePhotoSelectedState");
				if (props.onPhotoSelectionChanged && photoId) {
					const newSelection = props.selectedItems;

					if (selected) {
						newSelection.push(photoId);
					}
					else {
						const index = props.selectedItems.indexOf(photoId);
						if (index > -1) {
							newSelection.splice(index, 1);
						}
					}

					props.onPhotoSelectionChanged(newSelection);
				}
			};

			const onPhotoClick = (event: React.MouseEvent<HTMLDivElement, MouseEvent>) => {
				if (anySelected) {
					changePhotoSelectedState(props.selectedItems.indexOf(photoId) === -1);
				}
				else if (renderImageProps.onClick) {
					renderImageProps.onClick(event, {
						index: renderImageProps.index
					});
				}
			};

			const onPhotoSelectedChanged = (event: React.ChangeEvent<HTMLInputElement>) => {
				const checked = event.target.checked;
				changePhotoSelectedState(checked);
			};

			const onContextMenu = photosSelectable ? (event: React.MouseEvent<HTMLDivElement, MouseEvent>) => {
				event.preventDefault();

				changePhotoSelectedState(props.selectedItems.indexOf(photoId) === -1);
			} : undefined;

			return <div key={photoId} style={containerStyle} className={cssClass}>
				<input type="checkbox" id={checkboxId} name={checkboxId}
					checked={isSelected}
					onChange={onPhotoSelectedChanged}/>
				<label htmlFor={checkboxId} style={checkboxLabelStyle}></label>
				{/* Render a div instead of an img element. This is solely to prevent the default (longpress) context menu to appear in mobile browsers */}
				<div
					id={photoId}
					className="photoImg"
					style={imgStyle}
					onClick={onPhotoClick}
					onContextMenu={onContextMenu}
				/>
			</div>;
		}
		else {
			return null;
		}
	};

	return (
		<div className="photoGallery">
			<Gallery photos={galleryViewModel}
				onClick={(e, d) => { !!props.onClick && props.onClick(e, d);}}
				margin={PHOTO_MARGIN}
				targetRowHeight={PHOTO_HEIGHT}
				renderImage={imageRenderer}/>
		</div>
	);
};

export default PhotoGallery;