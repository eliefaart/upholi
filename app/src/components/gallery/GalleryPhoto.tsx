import * as React from "react";
import { FC } from "react";
import usePhotoThumbnailSource from "../../hooks/usePhotoThumbnailSource";

interface Props {
	photo: {
		key?: string,
		src: string,
		width: number,
		height: number
	},
	margin?: string,
	// top?: string,
	// left?: string,

	selected: boolean,
	anySiblingPhotoSelected: boolean,

	onClick: () => void,
	onToggleSelect: () => void,
}

const GalleryPhoto: FC<Props> = (props: Props) => {
	const photoId = props.photo.key ?? "";

	//const thumbnailSrc = usePhotoThumbnailSource(photoId);

	if (!photoId) {
		return <></>;
	}
	else {
		const cssClass = "photo"
			// + " " + (photosSelectable ? "selectable" : "")
			+ " " + (props.selected ? "selected" : "")
			+ " " + (props.anySiblingPhotoSelected ? "any-other-selected" : "")
			;

		const imgStyle: React.CSSProperties = {
			backgroundImage: "url(\"" + props.photo.src + "\")",
			margin: props.margin,
			width: props.photo.width,
			height: props.photo.height,
			// top: props.top,
			// left: props.left
		};

		const checkboxElementId = `photo_select_${photoId}`;

		return <div key={photoId} className={cssClass}>
			<input type="checkbox" id={checkboxElementId} name={checkboxElementId}
				checked={props.selected}
				// onChange={onPhotoSelectedChanged}
				/>
			<label htmlFor={checkboxElementId}></label>
			{/* Render a div instead of an img element. This is solely to prevent the default (longpress) context menu to appear in mobile browsers */}
			<div
				id={photoId}
				className="photoImg"
				style={imgStyle}
				onClick={props.onClick}
				onContextMenu={(event) => {
					event.preventDefault();
					props.onToggleSelect();
				}}
			/>
		</div>;
	}
};

export default GalleryPhoto;