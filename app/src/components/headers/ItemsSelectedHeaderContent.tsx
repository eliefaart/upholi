import * as React from "react";
import { FC } from "react";
import Button from "../misc/Button";
import { IconClose } from "../misc/Icons";

interface Props {
	selectedItems: string[],
	onSelectionCleared: () => void,
	actions?: JSX.Element | null,
}

const ItemsSelectedHeaderContent: FC<Props> = (props) => {

	if (props.selectedItems.length === 0) {
		return null;
	}
	else {
		return <>
			<div className="left">
				{props.actions}
			</div>
			<div className="right">
				{props.selectedItems.length > 0 && <Button
					onClick={() => props.onSelectionCleared()}
					label={props.selectedItems.length + " selected"}
					icon={<IconClose />}
					iconPosition="right"
				/>}
			</div>
		</>;
	}
};

export default ItemsSelectedHeaderContent;