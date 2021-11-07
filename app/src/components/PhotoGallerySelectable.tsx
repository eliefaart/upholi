import * as React from "react";
import { default as Gallery, RenderImageProps, PhotoProps} from "react-photo-gallery";
import GalleryPhoto from "../models/GalleryPhoto";

interface Props {
	photos: GalleryPhoto[],
	selectedItems: string[],
	onPhotoSelectedChange?: (photoId: string, selected: boolean) => void,
	onClick: (event: React.MouseEvent<Element, MouseEvent>, photo: { index: number }) => void
}

class PhotoGallerySelectable extends React.Component<Props> {

	constructor(props: Props) {
		super(props);
	}

	getGalleryViewModel(photos: GalleryPhoto[]): PhotoProps[] {
		const viewModel: PhotoProps[] = [];

		for (const photo of photos) {
			viewModel.push({
				key: photo.id,
				src: photo.src,
				width: photo.width,
				height: photo.height
			});
		}

		return viewModel;
	}

	render(): React.ReactNode {
		const photoHeight = 200;	// Target height for algorithm, but exact height will vary a bit.
		const photoMargin = 3;
		const photosSelectable = this.props.onPhotoSelectedChange !== undefined;

		const galleryViewModel = this.getGalleryViewModel(this.props.photos);


		// Inline-component representing one photo tile.
		//{ index: number, onClick, photo: Photo, margin: number, direction: string, top: number, left: number, key: string }
		const imageRenderer: React.FunctionComponent<RenderImageProps<Record<string, never>>> = (renderImageProps: RenderImageProps<Record<string, never>>) => {
			if (renderImageProps.photo.key) {
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

				const checkboxId = "photo_select_" + renderImageProps.photo.key;
				const isSelected = this.props.selectedItems.indexOf(renderImageProps.photo.key) !== -1;
				const anySelected = this.props.selectedItems.length > 0;
				const cssClass = "photo"
					+ " " + (photosSelectable ? "selectable" : "")
					+ " " + (isSelected ? "selected" : "")
					+ " " + (anySelected ? "any-other-selected" : "");

				const onPhotoClick = (event: React.MouseEvent<HTMLDivElement, MouseEvent>) => {
					if (anySelected) {
						if (this.props.onPhotoSelectedChange && renderImageProps.photo.key) {
							this.props.onPhotoSelectedChange(renderImageProps.photo.key, !isSelected);
						}
					}
					else if (renderImageProps.onClick) {
						renderImageProps.onClick(event, {
							index: renderImageProps.index
						});
					}
				};

				const onPhotoSelectedChanged = (event: React.ChangeEvent<HTMLInputElement>) => {
					const checked = event.target.checked;

					if (this.props.onPhotoSelectedChange && renderImageProps.photo.key) {
						this.props.onPhotoSelectedChange(renderImageProps.photo.key, checked);
					}
				};

				const onContextMenu = photosSelectable ? (event: React.MouseEvent<HTMLDivElement, MouseEvent>) => {
					event.preventDefault();
					if (this.props.onPhotoSelectedChange && renderImageProps.photo.key) {
						this.props.onPhotoSelectedChange(renderImageProps.photo.key, !isSelected);
					}
				} : undefined;

				return <div key={renderImageProps.photo.key} style={containerStyle} className={cssClass}>
					<input type="checkbox" id={checkboxId} name={checkboxId}
						checked={isSelected}
						onChange={onPhotoSelectedChanged}/>
					<label htmlFor={checkboxId} style={checkboxLabelStyle}></label>
					{/* Render a div instead of an img element. This is solely to prevent the default (longpress) context menu to appear in mobile browsers */}
					<div
						id={renderImageProps.photo.key}
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
					onClick={(e, d) => { !!this.props.onClick && this.props.onClick(e, d);}}
					margin={photoMargin}
					targetRowHeight={photoHeight}
					renderImage={imageRenderer}/>
			</div>
		);
	}
}

export default PhotoGallerySelectable;