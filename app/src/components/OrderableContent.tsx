import * as React from "react";

interface Props {
	className?: string;
	onOrderChanged: (movedItemKey: string, newPosition: number) => void;
}

interface State {
	items: Items;
}

interface Item {
	key: string;
	originalElement: React.ReactElement<unknown>;
	originalIndex: number;

	elementRef: React.RefObject<HTMLInputElement>;
	element: JSX.Element;

	positionTopLeftX: number;
	positionTopLeftY: number;
	positionBottomRightX: number;
	positionBottomRightY: number;
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
				if (!child.key) {
					throw Error(`Child ${child} does not have a key.`);
				}
				items.push({
					key: String(child.key),
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

	getElements(): JSX.Element[] {
		return this.items.map(item => item.element);
	}

	getItemAtPosition(posX: number, posY: number): Item | null {
		const item = this.items.find(it =>
			it.positionTopLeftX < posX && it.positionBottomRightX > posX
			&& it.positionTopLeftY < posY && it.positionBottomRightY > posY);

		return item || null;
	}

	updateItemPositions(container: React.RefObject<HTMLInputElement>): void {
		if (!!container?.current && this.items.length > 0) {
			const containerRect = container.current.getBoundingClientRect();

			for (const item of this.items) {
				const element = item.elementRef.current;
				if (element) {
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

export default class OrderableContent extends React.Component<Props, State> {

	containerRef: React.RefObject<HTMLInputElement> | null;

	dragTarget: Item | null;
	lastDragPosX: number | null;
	lastDragPosY: number | null;

	constructor(props: Props) {
		super(props);

		this.containerRef = null;
		this.dragTarget = null;
		this.lastDragPosX = null;
		this.lastDragPosY = null;

		this.onDragStart = this.onDragStart.bind(this);
		this.onDragOver = this.onDragOver.bind(this);
		this.onDragEnd = this.onDragEnd.bind(this);

		this.state = {
			items: Items.from(this.props.children)
		};
	}

	componentDidMount(): void {
		if (this.containerRef) {
			this.state.items.updateItemPositions(this.containerRef);
		}
	}

	componentDidUpdate(): void {
		// Figure out if parent element has updated the child elements.
		// If so, then update the items in the state.
		let childrenUpdated = this.state.items.items.length !== React.Children.count(this.props.children);
		React.Children.forEach(this.props.children, (child, index) => {
			const childElement = child as React.ReactElement;
			const childKey = childElement ? String(childElement.key) : "";
			childrenUpdated = childrenUpdated || this.state.items.items[index].key !== childKey;
		});

		if (childrenUpdated) {
			this.setState({
				items: Items.from(this.props.children)
			});
		}

		if (this.containerRef) {
			this.state.items.updateItemPositions(this.containerRef);
		}
	}

	onDragStart(event: React.DragEvent): void {
		// Keep track of the Item being dragged.
		const coords = this.getDragEventXY(event);
		if (coords) {
			this.dragTarget = this.state.items.getItemAtPosition(coords.x, coords.y);
		}
	}

	onDragOver(event: React.DragEvent): void {
		const coords = this.getDragEventXY(event);
		if (coords) {
			this.lastDragPosX = coords.x;
			this.lastDragPosY = coords.y;
		}
	}

	onDragEnd(): void {
		if (this.dragTarget && this.lastDragPosX && this.lastDragPosY) {
			const originalItem = this.dragTarget;
			const targetItem = this.state.items.getItemAtPosition(this.lastDragPosX, this.lastDragPosY);

			if (targetItem) {
				const targetIndex = this.state.items.items.indexOf(targetItem);
				this.props.onOrderChanged(originalItem.key, targetIndex );
			}
		}

		this.dragTarget = null;
		this.lastDragPosX = null;
		this.lastDragPosY = null;
	}

	getDragEventXY(event: React.DragEvent): {x: number, y: number} | null {
		if (this.containerRef?.current) {
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

	render(): React.ReactNode {
		this.containerRef = React.createRef<HTMLInputElement>();
		const itemElements = this.state.items.items.map(item => item.element);
		const className = `orderable-content ${this.props.className || ""}`.trim();

		return <div
			ref={this.containerRef}
			onDragStart={this.onDragStart}
			onDragOver={this.onDragOver}
			onDragEnd={this.onDragEnd}
			className={className}>
			{itemElements}
		</div>;
	}
}