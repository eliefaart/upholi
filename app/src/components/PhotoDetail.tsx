import * as React from "react";
import ExifData from "./misc/ExifData";
import Exif from "../models/Exif";

interface PhotoDetailProps {
	src: string;
	isVideo: boolean;
	exif: Exif | null;
}

interface PhotoDetailState {
	isPanning: boolean;
}

const ZoomStyleEnum = {
	// Using delta in pixels
	BY_DELTA: 0,
	// Using fixed step, some percentage
	FIXED_STEPS: 1
};

class PhotoDetail extends React.Component<PhotoDetailProps, PhotoDetailState> {
	constructor(props: PhotoDetailProps) {
		super(props);

		this.state = {
			isPanning: false
		};
	}

	componentDidMount(): void {
		const containerElement = document.getElementsByClassName("photo-detail")[0] as HTMLElement;
		const imgElement = document.getElementsByClassName("photoDetailImg")[0] as HTMLElement;

		// Zoom on mousewheel
		containerElement.onwheel = (event) => {
			this.zoomPhoto(imgElement, event.deltaY, ZoomStyleEnum.FIXED_STEPS);
		};

		// Pan photo on mouse or touch move
		let panLastX: number, panLastY: number;
		let isPanning = false;
		const fnStartPanning = (event: MouseEvent | TouchEvent) => {
			isPanning = true;

			const coords = this.getClickCoordinatesFromEvent(event);
			panLastX = coords.x;
			panLastY = coords.y;
		};

		const fnStopPanning = () => {
			isPanning = false;
			isTouchZooming = false;
		};

		const fnOnTouchMove = (event: TouchEvent) => {
			fnHandlePanning(event);
			fnHandlePinching(event);
		};

		// Handle panning, moving image along x and y axis
		const fnHandlePanning = (event: MouseEvent | TouchEvent) => {
			if (isPanning) {
				const coords = this.getClickCoordinatesFromEvent(event);
				const currentX = coords.x;
				const currentY = coords.y;

				const deltaX = currentX - panLastX;
				const deltaY = currentY - panLastY;

				this.movePhoto(imgElement, deltaX, deltaY);

				panLastX = currentX;
				panLastY = currentY;
			}
		};

		// Handle touch pinching: zooming
		let fingerDistanceLast: number;
		let isTouchZooming = false;
		const fnHandlePinching = (event: TouchEvent) => {
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
					this.zoomPhoto(imgElement, delta, ZoomStyleEnum.BY_DELTA);
				}

				isTouchZooming = true;
				fingerDistanceLast = fingerDistance;
			}
		};

		// Handle double click - reset zoom
		const fnOnDoubleClick = () => {
			this.resetView(imgElement);
		};

		imgElement.ondragstart = () => false;
		imgElement.ondrop = () => false;

		containerElement.onmousedown = fnStartPanning;
		containerElement.ontouchstart = fnStartPanning;

		containerElement.onmouseup = fnStopPanning;
		containerElement.onmouseleave = fnStopPanning;
		containerElement.ontouchend = fnStopPanning;

		containerElement.onmousemove = fnHandlePanning;
		containerElement.ontouchmove = fnOnTouchMove;

		containerElement.ondblclick = fnOnDoubleClick;
	}

	// Get the X and Y click coordinates for event,
	// wether it is from a mouse or touch event
	getClickCoordinatesFromEvent(event: MouseEvent | TouchEvent): { x: number, y: number } {
		let x = 0;
		let y = 0;

		// Get X and Y in a different way depending on wether it is a mouse or touch event.
		if (event instanceof MouseEvent) {
			x = event.clientX;
			y = event.clientY;
		}
		else if (event instanceof TouchEvent) {
			const touches = event.touches;

			const fnTouchListToArray = (touchList: TouchList) => {
				const array = [];
				for (let i = 0; i < touchList.length; i++) {
					array.push(touchList[i]);
				}
				return array;
			};
			const fnAverage = (numbers: number[]): number => {
				let total = 0;
				for (const number of numbers) {
					total += number;
				}
				return total / numbers.length;
			};

			if (!!touches && touches.length > 0) {
				// Calculate the average of all touch points
				x = fnAverage(fnTouchListToArray(touches).map(t => t.clientX));
				y = fnAverage(fnTouchListToArray(touches).map(t => t.clientY));
			}
		}

		return { x, y };
	}

	// Zoom the image by given number of units
	zoomPhoto(imgElement: HTMLElement, zoomDelta: number, zoomStyle: number): void {
		if (!zoomDelta || zoomDelta === 0)
			return;

		let zoomStepPercentage = 15;	// Default step
		const zoomingIn = zoomDelta < 0;

		// Find current scale factor
		const matches = /scale\((.+)\)/.exec(imgElement.style.transform);
		const currentScaleFactor = matches && matches.length >= 2
			? parseFloat(matches[1])
			: 1;

		if (zoomStyle == ZoomStyleEnum.BY_DELTA) {
			zoomStepPercentage = Math.abs((zoomDelta / imgElement.clientWidth) * 100);
		}

		// Calculate new scale factor
		const zoomStep = (currentScaleFactor / 100) * zoomStepPercentage;
		const newScaleFactor = zoomingIn
			? currentScaleFactor + zoomStep
			: currentScaleFactor - zoomStep;

		// Set new scale factor
		imgElement.style.transform = "scale(" + newScaleFactor + ")";
	}

	// reset the photo to its default position and zoom level
	resetView(imgElement: HTMLElement): void {
		imgElement.style.transform = "scale(1)";
		imgElement.style.left = "0px";
		imgElement.style.top = "0px";
	}

	// Move/pan the image by given number of units.
	movePhoto(imgElement: HTMLElement, deltaX: number, deltaY: number): void {
		if (imgElement.parentElement) {
			const imgWidth = imgElement.getBoundingClientRect().width;
			const imgHeight = imgElement.getBoundingClientRect().height;
			const containerWidth = imgElement.parentElement.getBoundingClientRect().width;
			const containerHeight = imgElement.parentElement.getBoundingClientRect().height;

			const imgFitsInContainerX = imgWidth <= containerWidth;
			const imgFitsInContainerY = imgHeight <= containerHeight;

			// Image smaller than container, X-axis
			if (imgFitsInContainerX && deltaX !== 0) {
				const movingRight = deltaX > 0;
				const movingLeft = !movingRight;

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
				const movingDown = deltaY > 0;
				const movingUp = !movingDown;

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
				const movingRight = deltaX > 0;
				const movingLeft = !movingRight;

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
				const movingDown = deltaY > 0;
				const movingUp = !movingDown;

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
	}

	render(): React.ReactNode {
		return <div className="photo-detail">
			{this.props.exif != null && <ExifData exif={this.props.exif} />}
			{this.props.isVideo
				? <video src={this.props.src}
					className="photoDetailVideo"
					draggable={false}
					style={{ top: "0px", left: "0px", maxWidth: "90vw", maxHeight: "90vw" }}
					controls />
				: <img src={this.props.src}
					className="photoDetailImg"
					draggable={false}
					style={{ top: "0px", left: "0px" }} />
			}

		</div>;
	}
}

export default PhotoDetail;