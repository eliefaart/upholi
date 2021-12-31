import * as React from "react";
import { FC } from "react";
import { IconClose } from "../misc/Icons";

interface Props {
	selectedItems: string[],
	onSelectionCleared: () => void
}

const ItemsSelectedHeaderContent: FC<Props> = (props) => {

	if (props.selectedItems.length === 0) {
		return null;
	}
	else {
		return <div id="header-items-selected">
			{props.selectedItems.length > 0 && <button
				className="with-icon"
				onClick={() => props.onSelectionCleared()}
				title={props.selectedItems.length + "selected"}>
				{props.selectedItems.length} selected<IconClose />
			</button>}
		</div>;
	}
};

export default ItemsSelectedHeaderContent;