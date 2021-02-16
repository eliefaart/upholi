import * as React from "react";

interface Props {
	className?: string,
	onOrderChanged: () => void
}

interface State {
}

interface Item {
	elementRef: React.RefObject<HTMLInputElement>,

	positionTopLeftX: number,
	positionTopLeftY: number,
	positionBottomRightX: number,
	positionBottomRightY: number,
}

export default class OrderableContent extends React.Component<Props, State> {

	containerRef: React.RefObject<HTMLInputElement> | null;
	items: Item[];

	constructor(props: Props) {
		super(props);

		this.containerRef = null;
		this.items = [];

		this.onDragEnd = this.onDragEnd.bind(this);
		this.onDragOver = this.onDragOver.bind(this);

		this.state = {
		};
	}

	componentDidUpdate() {
		this.updateItemPositions();

		// window.requestAnimationFrame(function() {
		// 	// Note: I may also need setTimeout to ensure this gets executed
		// 	// after browser fully finished rendering.
		// 	// https://stackoverflow.com/questions/26556436/react-after-render-code
		// });
	}

	onDragOver(event: React.DragEvent) {
		if (!!this.containerRef?.current) {
			const containerRect = this.containerRef.current.getBoundingClientRect();
			const posX = event.clientX - containerRect.x;
			const posY = event.clientY - containerRect.y;
			console.log(posX, posY);
		}
	}

	onDragEnd(event: React.DragEvent) {
		console.log("Drop!");
	}

	/**
	 */
	updateItemPositions() {
		if (!!this.containerRef?.current && this.items.length > 0) {
			const containerRect = this.containerRef.current.getBoundingClientRect();

			for (const item of this.items) {
				const element = item.elementRef.current;
				if (!!element) {
					const rect = element.getBoundingClientRect();
					const posX = rect.x - containerRect.x;
					const posY = rect.y - containerRect.y;

					item.positionTopLeftX = posX;
					item.positionTopLeftY = posY;
					item.positionBottomRightX = posX + rect.width;
					item.positionBottomRightY = posY + rect.height;
				}
			}
		}

		console.log(this.items);
	}

	getItemAtPosition(posX: number, poxY: number): Item | null {
		return null;
	}

	render() {
		this.containerRef = React.createRef<HTMLInputElement>();
		this.items = [];

		const _this = this;

		const items = React.Children.map(this.props.children, child => {
			if (React.isValidElement(child)) {
				const elementRef = React.createRef<HTMLInputElement>();
				_this.items.push({
					elementRef: elementRef,
					// Position is unknown at this point.
					positionTopLeftX: 0,
					positionTopLeftY: 0,
					positionBottomRightX: 0,
					positionBottomRightY: 0
				});

				return React.cloneElement(child, {
					ref: elementRef,
					draggable: true,
					onDragEnd: this.onDragEnd
				});
			}
			return child;
		});

		return <div
			ref={this.containerRef}
			onDragOver={this.onDragOver}
			className={this.props.className}>
			{items}
		</div>;
	}
}