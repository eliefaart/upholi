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
		let galleryColumns = 2;
		if (width >= 900)
			galleryColumns = 3;
		if (width >= 1200)
			galleryColumns = 4;
		if (width >= 1500)
			galleryColumns = 5;
		if (width >= 1800)
			galleryColumns = 6;

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

			// This boolean's initial value is false. It will be set to true when the click-start event fires.
			// If the value of this boolean is false during the event handler of the click(end) event,
			// then React has rerendered this component while the mouse/touch was held down.
			// This can happen because if the mouse button is held down long enough, the photo is selected, instead of opened. 
			// Selecting the photo changes the state; and causes this component to be rerendered.
			let clickStartEventOccuredDuringCurrentComponentLifespan = false;
			let longPressTimer, longPressTouchStartX, longPressTouchStartY;
			const onPhotoClickStart = event => {
				event.persist();

				const longPressDelay = 600;
				clickStartEventOccuredDuringCurrentComponentLifespan = true;

				longPressTouchStartX = event.clientX || event.touches[0].clientX;
				longPressTouchStartY = event.clientY || event.touches[0].clientY;

				longPressTimer = setTimeout(fnSelectPhoto, longPressDelay);
			};
			const cancelLongPress = () => {
				clearTimeout(longPressTimer);
				longPressTimer = null;
			}

			const onTouchMove = event => {
				// Cancel the longPress timer if user has moved too much while holding touch down;
				// because likely he is just scrolling the page
				const longPressMoveTolerance = 35;
				const startX = event.touches[0].clientX;
				const startY = event.touches[0].clientY;

				const fnMovedTooMuch = (x1, x2) => Math.abs(x1 - x2) > longPressMoveTolerance;

				if (longPressTimer != null &&
					(fnMovedTooMuch(longPressTouchStartX, startX) || fnMovedTooMuch(longPressTouchStartY, startY)))  {
					cancelLongPress();
				}
			}

			const onPhotoClick = event => {
				// If the click-start event did not fire during current lifespan of this component, 
				// then we do not need to handle the click event,
				// because the user's interactionw as handled by the timeout event handler
				if (clickStartEventOccuredDuringCurrentComponentLifespan !== true)
					return;

				cancelLongPress();

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
						onMouseDown={onPhotoClickStart}
						onTouchStart={onPhotoClickStart}
						onMouseLeave={cancelLongPress}
						onDragStart={cancelLongPress}
						onTouchMove={onTouchMove}
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