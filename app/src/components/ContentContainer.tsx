import * as React from "react";

interface ContentContainerProps {
	className?: string,
	paddingTop?: boolean,
	onDrop?: (event: React.DragEvent<HTMLElement>) => void,
	onDragOver?: (event: React.DragEvent<HTMLElement>) => void,
}

/**
 * I feel like I don't really need this class..
 * Perhaps I can include this functionality in PageBaseComponent somehow? Or AppBody?
 */
class ContentContainer extends React.Component<ContentContainerProps> {

	constructor(props: ContentContainerProps) {
		super(props);
	}

	getClassName(): string | undefined {
		let className = this.props.className || "";

		if (this.props.paddingTop === true){
			className += " padding-top";
		}

		className = className.trim();
		if (className === ""){
			return undefined;
		}
		else {
			return className;
		}
	}

	render(): React.ReactNode {
		const className = this.getClassName();

		return (<main id="content"
			className={className}
			onDrop={this.props.onDrop}
			onDragOver={this.props.onDragOver || ((event) => event.preventDefault())}>
			{this.props.children}
		</main>);
	}
}

export default ContentContainer;