import * as React from "react";
import { FC } from "react";

interface Props {
	className?: string,
	paddingTop?: boolean,
	onDrop?: (event: React.DragEvent<HTMLElement>) => void,
	onDragOver?: (event: React.DragEvent<HTMLElement>) => void,
}

/**
 * I feel like I don't really need this component..
 * Perhaps I can include this functionality in PageBaseComponent somehow? Or AppBody?
 */
const ContentContainer: FC<Props> = (props) => {
	let className: string | undefined = props.className || "";

	if (props.paddingTop === true){
		className += " padding-top";
	}

	if (className.trim() === ""){
		className = undefined;
	}

	return (<main id="content"
		className={className}
		onDrop={props.onDrop}
		onDragOver={props.onDragOver || ((event) => event.preventDefault())}>
		{props.children}
	</main>);
};

export default ContentContainer;