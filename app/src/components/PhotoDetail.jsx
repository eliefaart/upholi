import React from 'react';
import ExifData from '../components/ExifData.jsx';

class PhotoDetail extends React.Component {
	constructor(props) {
		super(props);

		this.state = {
			isPanning: false
		};
	}

	componentDidMount() {
		const imgElement = document.getElementsByClassName("photoDetailImg")[0];

		// Zoom on mousewheel
		imgElement.onwheel = (event) => {
			this.zoomPhoto(imgElement, event.deltaY);
		};

		// Pan photo on mouse or touch move
		let panLastX, panLastY;
		let fingerDistanceLast;
		let isTouchZooming = false;
		const fnStartPanning = (event) => {
			this.setState({isPanning: true });

			const coords = this.getClickCoordinatesFromEvent(event);
			panLastX = coords.x;
			panLastY = coords.y;
		};
		const fnStopPanning = () => this.setState({isPanning: false });
		const fnOnTouchMove = (event) => {
			fnOnPan(event);
			fnTouchZoom(event);
		}
		const fnOnPan = (event) => {
			if (this.state.isPanning) {
				const coords = this.getClickCoordinatesFromEvent(event);
				let currentX = coords.x;
				let currentY = coords.y;

				let deltaX = currentX - panLastX;
				let deltaY = currentY - panLastY;

				this.movePhoto(imgElement, deltaX, deltaY);

				panLastX = currentX;
				panLastY = currentY;
			}
		};
		const fnTouchZoom = (event) => {
			const touches = event.touches;
			if (!!touches && touches.length >= 2) {

				// Only take the first two touches into account for now
				const fingerDistance = Math.sqrt(
					Math.pow(touches[1].clientX - touches[0].clientX, 2) +
					Math.pow(touches[1].clientY - touches[0].clientY, 2)
				);
				
				// If user was already touch-zooming, then handle zoom
				// Otherwise do nothing until next touch event.
				if (isTouchZooming) {
					const delta = fingerDistanceLast - fingerDistance;
					this.zoomPhoto(imgElement, delta);
				}

				fingerDistanceLast = fingerDistance;
			}
		}

		imgElement.onmousedown = fnStartPanning;
		imgElement.ontouchstart = fnStartPanning;

		imgElement.onmouseup = fnStopPanning;
		imgElement.onmouseleave = fnStopPanning;
		imgElement.ontouchend = fnStopPanning;

		imgElement.onmousemove = fnOnPan;
		imgElement.ontouchmove = fnOnTouchMove;
	}

	getClickCoordinatesFromEvent(event) {
		let x = event.clientX;
		let y = event.clientY;

		// If this event is a touch event, get X and Y in a different way.
		const touches = event.touches;
		
		const fnTouchListToArray = (touchList) => {
			let array = [];
			for (let i = 0; i < touchList.length; i++) {
				array.push(touchList[i]);
			}
			return array;
		}
		const fnAverage = (numbers) => {
			let total = 0;
			for (let number of numbers) {
				total += number;
			}
			return total / numbers.length;
		}

		if (!!touches && touches.length > 0) {
			x = fnAverage(fnTouchListToArray(touches).map(t => t.clientX));
			y = fnAverage(fnTouchListToArray(touches).map(t => t.clientY));
		}

		return { x, y };
	}

	// Zoom the image by given number of units
	zoomPhoto(imgElement, zoomDelta) {
		const zoomStepPercentage = 10;
		const zoomingIn = zoomDelta < 0;

		// Find current scale factor
		let matches = /scale\((.+)\)/.exec(imgElement.style.transform);
		const currentScaleFactor = matches && matches.length >= 2
			? parseFloat(matches[1])
			: 1;

		// Calculate new scale factor
		const zoomStep = (currentScaleFactor / 100) * zoomStepPercentage;
		let newScaleFactor = zoomingIn 
			? currentScaleFactor + zoomStep 
			: currentScaleFactor - zoomStep;

		// Set new scale factor
		imgElement.style.transform = "scale(" + newScaleFactor + ")";
	}

	// Move/pan the image by given number of units.
	movePhoto(imgElement, deltaX, deltaY) {
		const imgWidth = imgElement.getBoundingClientRect().width;
		const imgHeight = imgElement.getBoundingClientRect().height;
		const containerWidth = imgElement.parentElement.getBoundingClientRect().width;
		const containerHeight = imgElement.parentElement.getBoundingClientRect().height;

		const imgFitsInContainerX = imgWidth <= containerWidth;
		const imgFitsInContainerY = imgHeight <= containerHeight;
		
		// Image smaller than container, X-axis
		if (imgFitsInContainerX && deltaX !== 0) {
			let movingRight = deltaX > 0;
			let movingLeft = !movingRight;

			if (movingRight) {
				const availablePixels = -(imgElement.getBoundingClientRect().right - imgElement.parentElement.getBoundingClientRect().width);
				
				if (deltaX > availablePixels) {
					deltaX = availablePixels;
				}
			}
			if (movingLeft) {
				const availablePixels = imgElement.getBoundingClientRect().left;
				
				if (-deltaX > availablePixels) {
					deltaX = -availablePixels;
				}
			}
		}

		// Image smaller than container, Y-axis
		if (imgFitsInContainerY && deltaY !== 0) {
			let movingDown = deltaY > 0;
			let movingUp = !movingDown;

			if (movingDown) {
				const availablePixels = -(imgElement.getBoundingClientRect().bottom - imgElement.parentElement.getBoundingClientRect().height);
				
				if (deltaY > availablePixels) {
					deltaY = availablePixels;
				}
			}
			if (movingUp) {
				const availablePixels = imgElement.getBoundingClientRect().top;
				
				if (-deltaY > availablePixels) {
					deltaY = -availablePixels;
				}
			}
		}

		// Image larger than container, X-axis
		if (!imgFitsInContainerX && deltaX !== 0) {
			let movingRight = deltaX > 0;
			let movingLeft = !movingRight;

			if (movingRight) {
				const availablePixels = -imgElement.getBoundingClientRect().left;
				
				if (deltaX > availablePixels) {
					deltaX = availablePixels;
				}
			}
			if (movingLeft) {
				const availablePixels = -(imgElement.parentElement.getBoundingClientRect().width - imgElement.getBoundingClientRect().right);
				
				if (-deltaX > availablePixels) {
					deltaX = -availablePixels;
				}
			}
		}

		// Image larger than container, Y-axis
		if (!imgFitsInContainerY && deltaY !== 0) {
			let movingDown = deltaY > 0;
			let movingUp = !movingDown;

			if (movingDown) {
				const availablePixels = -imgElement.getBoundingClientRect().top;
				
				if (deltaY > availablePixels) {
					deltaY = availablePixels;
				}
			}
			if (movingUp) {
				const availablePixels = -(imgElement.parentElement.getBoundingClientRect().height - imgElement.getBoundingClientRect().bottom);
				
				if (-deltaY > availablePixels) {
					deltaY = -availablePixels;
				}
			}
		}

		// Get current top and left values
		const currentTop = parseFloat(imgElement.style.top.replace("px", ""));
		const currentLeft = parseFloat(imgElement.style.left.replace("px", ""));

		// Set new top and left values
		imgElement.style.left = (currentLeft + deltaX) + "px";
		imgElement.style.top = (currentTop + deltaY) + "px";
	}

	render() {
		return <div className="photoDetail">
			{this.props.exif && !this.state.isPanning && <ExifData exif={this.props.exif}/>}
			<img className="photoDetailImg" src={this.props.src} draggable={false}
				style={{top: "0px", left: "0px"}}/>
		</div>;
	}
}

export default PhotoDetail;