import * as React from "react";

interface Props {
	className?: string,
	onOrderChanged: (movedItem: React.ReactElement<any>, newPosition: number) => void
}

interface State { }

interface Item {
	originalElement: React.ReactElement<any>,
	originalIndex: number,

	elementRef: React.RefObject<HTMLInputElement>,
	element: JSX.Element,

	positionTopLeftX: number,
	positionTopLeftY: number,
	positionBottomRightX: number,
	positionBottomRightY: number,
}

class Items {
	items: Item[];

	constructor(items: Item[]) {
		this.items = items;
	}

	static from(reactNode: React.ReactNode): Items {
		const items: Item[] = [];

		React.Children.forEach(reactNode, child => {
			if (React.isValidElement(child)) {
				const elementRef = React.createRef<HTMLInputElement>();
				items.push({
					originalElement: child,
					originalIndex: items.length,
					elementRef: elementRef,
					element: React.cloneElement(child, {
						ref: elementRef,
						draggable: true
					}),
					// Position is unknown at this point.
					positionTopLeftX: 0,
					positionTopLeftY: 0,
					positionBottomRightX: 0,
					positionBottomRightY: 0
				});
			}
		});

		return new Items(items);
	}

	getItemAtPosition(posX: number, posY: number): Item | null {
		const item = this.items.find(it =>
			it.positionTopLeftX < posX && it.positionBottomRightX > posX
			&& it.positionTopLeftY < posY && it.positionBottomRightY > posY);

		return item || null;
	}

	updateItemPositions(container: React.RefObject<HTMLInputElement>) {
		if (!!container?.current && this.items.length > 0) {
			const containerRect = container.current.getBoundingClientRect();

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
	}
}

export default class OrderableContent extends React.Component<Props> {

	containerRef: React.RefObject<HTMLInputElement> | null;
	items: Items;

	dragTarget: Item | null;
	lastDragPosX: number | null;
	lastDragPosY: number | null;

	constructor(props: Props) {
		super(props);

		this.containerRef = null;
		this.items = Items.from(this.props.children);
		this.dragTarget = null;
		this.lastDragPosX = null;
		this.lastDragPosY = null;

		this.onDragStart = this.onDragStart.bind(this);
		this.onDragOver = this.onDragOver.bind(this);
		this.onDragEnd = this.onDragEnd.bind(this);

		this.state = {
		};
	}

	componentDidMount() {
		if (this.containerRef) {
			this.items.updateItemPositions(this.containerRef);
		}
	}

	componentDidUpdate(){
		if (this.containerRef) {
			this.items.updateItemPositions(this.containerRef);
		}

		// const fnUpdateItemPositions = this.updateItemPositions;
		// window.requestAnimationFrame(function() {
		// 	// Note: I may also need setTimeout to ensure this gets executed
		// 	// after browser fully finished rendering.
		// 	// https://stackoverflow.com/questions/26556436/react-after-render-code
		// 	setTimeout(function () {
		// 		fnUpdateItemPositions();
		// 	})
		// });
	}

	onDragStart(event: React.DragEvent) {
		// Keep track of the Item being dragged.
		const coords = this.getDragEventXY(event);
		if (coords) {
			this.dragTarget = this.items.getItemAtPosition(coords.x, coords.y);
		}
	}

	onDragOver(event: React.DragEvent) {
		const coords = this.getDragEventXY(event);
		if (coords) {
			this.lastDragPosX = coords.x;
			this.lastDragPosY = coords.y;
		}
	}

	onDragEnd(event: React.DragEvent) {
		if (this.dragTarget && this.lastDragPosX && this.lastDragPosY) {
			const originalItem = this.dragTarget;
			const targetItem = this.items.getItemAtPosition(this.lastDragPosX, this.lastDragPosY);

			if (targetItem) {
				const targetIndex = this.items.items.indexOf(targetItem);
				this.props.onOrderChanged(originalItem.originalElement, targetIndex );
			}
		}

		this.dragTarget = null;
		this.lastDragPosX = null;
		this.lastDragPosY = null;
	}

	getDragEventXY(event: React.DragEvent): {x: number, y: number} | null {
		if (!!this.containerRef?.current) {
			const containerRect = this.containerRef.current.getBoundingClientRect();
			const posX = event.clientX - containerRect.x;
			const posY = event.clientY - containerRect.y;

			return {
				x: posX,
				y: posY
			};
		}
		else {
			return null;
		}
	}

	render() {
		this.containerRef = React.createRef<HTMLInputElement>();
		const items = this.items.items.map(item => item.element);
		const className = `orderable-content ${this.props.className || ""}`.trim();

		return <div
			ref={this.containerRef}
			onDragStart={this.onDragStart}
			onDragOver={this.onDragOver}
			onDragEnd={this.onDragEnd}
			className={className}>
			{items}
		</div>;
	}
}