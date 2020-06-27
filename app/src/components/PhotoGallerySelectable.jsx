import React, {useState, useCallback} from 'react';
import $ from 'jquery';
import Gallery from "react-photo-gallery";

class PhotoGallerySelectable extends React.Component {

	constructor(props) {
		super(props);

		this.state = {
		};
	}

	render() {
		let galleryComponent = this;

		// Todo: handle resize event to update column count
		const width = $("body").width();
		const roughWidthPerPhoto = 300;		// How width each photo should be, roughly (in pixels)
		const galleryColumns = Math.ceil(width / roughWidthPerPhoto);

		// Inline-component representing one photo tile.
		const imageRenderer = ({ index, onClick, photo, margin, direction, top, left, key }) => {
			const imgStyle = { 
				margin: margin, 
				display: "block",

				backgroundImage: "url(\"" + photo.src + "\")",
				backgroundSize: "cover",
				backgroundPosition: "center center",
				backgroundRepeat: "no-repeat",
				width: photo.width,
				height: photo.height,
			};
			if (direction === "column") {
				imgStyle.position = "absolute";
				imgStyle.left = left;
				imgStyle.top = top;
			}
		  
			const containerStyle = {
				position: "relative"
			};

			const checkboxLabelStyle = {
				position: "absolute",
				top: 0 + (margin * 2),
				left: 0 + (margin * 2),
			};

			const checkboxId = "photo_select_" + photo.id;
			const isSelected = galleryComponent.props.selectedItems.indexOf(photo.id) !== -1;
			const anySelected = galleryComponent.props.selectedItems.length > 0;
			const cssClass = "photo"
				+ " " + (isSelected ? "selected" : "")
				+ " " + (anySelected ? "any-other-selected" : "");

			const onPhotoClick = event => {
				if (anySelected) {
					if (galleryComponent.props.onPhotoSelectedChange) {
						galleryComponent.props.onPhotoSelectedChange(photo.id, !isSelected);
					}
				}
				else if (onClick) {
					onClick(event, { photo, index });
				}
			}

			const onPhotoSelectedChanged = event => {
				let checked = event.target.checked;
				
				if (galleryComponent.props.onPhotoSelectedChange) {
					galleryComponent.props.onPhotoSelectedChange(photo.id, checked);
				}
			};

			const onContextMenu = event => {
				event.preventDefault();
				galleryComponent.props.onPhotoSelectedChange(photo.id, !isSelected)
			}

			return (
				<div key={key} style={containerStyle} className={cssClass}>
					<input type="checkbox" id={checkboxId} name={checkboxId} 
						checked={isSelected} 
						onChange={onPhotoSelectedChanged}/>
					<label htmlFor={checkboxId} style={checkboxLabelStyle}></label>
					{/* Render a div instead of an img element. This is solely to prevent the default (longpress) context menu to appear in mobile browsers */}
					<div
						id={photo.id}
						className="photoImg"
						style={imgStyle}
						onClick={onPhotoClick}
						onContextMenu={onContextMenu}
					/>
				</div>
			);
		};

		return (
			<div className="photoGallery">
				<Gallery className="" photos={this.props.photos} onClick={(e, d) => { !!this.props.onClick && this.props.onClick(e, d);}} columns={galleryColumns} margin={3} renderImage={imageRenderer}/>
			</div>
		);
	}
}

export default PhotoGallerySelectable;