import React, {useState, useCallback} from 'react';
import $ from 'jquery';
import Gallery from "react-photo-gallery";

class PhotoGallerySelectable extends React.Component {

	constructor(props) {
		super(props);

		this.state = {
		};
	}

	componentDidMount() {
	}

	componentWillUnmount() {
	}
	
	render() {
		let galleryComponent = this;

		// Todo: handle resize event to update column count
		const width = $("body").width();
		let galleryColumns = 3;
		if (width >= 900)
			galleryColumns = 4;
		if (width >= 1200)
			galleryColumns = 5;
		if (width >= 1500)
			galleryColumns = 6;
		if (width >= 1800)
			galleryColumns = 7;

		// Inline-component representing one photo tile.
		const imageRenderer = ({ index, onClick, photo, margin, direction, top, left, key }) => {
			const imgStyle = { margin: margin, display: 'block' };
			if (direction === 'column') {
			  imgStyle.position = 'absolute';
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

			const fnSelectPhoto = () => {
				if (!isSelected && galleryComponent.props.onPhotoSelectedChange) {
					galleryComponent.props.onPhotoSelectedChange(photo.id, true);
				}
			}

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
				event.stopImmediatePropagation();
				galleryComponent.props.onPhotoSelectedChange(photo.id, !isSelected)
			}

			return (
				<div key={key} style={containerStyle} className={cssClass}>
					<input type="checkbox" id={checkboxId} name={checkboxId} 
						checked={isSelected} 
						onChange={onPhotoSelectedChanged}/>
					<label htmlFor={checkboxId} style={checkboxLabelStyle}></label>
					<img
						{...photo}
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