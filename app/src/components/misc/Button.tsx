import * as React from "react";

interface Props {
	label: string,
	icon?: React.ReactElement,
	/** Place icon at right or left side of label? Defaults to left. */
	iconPosition?: "right" | "left",
	onClick: () => void,
}

const Button = (props: Props): React.ReactElement => {
	const className = "button"
		+ (props.icon ? " with-icon" : "")
		+ (props.icon && props.iconPosition !== "right" ? " icon-left" : "")
		+ (props.icon && props.iconPosition === "right" ? " icon-right" : "");

	return <button className={className}
		onClick={props.onClick}
		title={props.label}>
		{props.icon && props.iconPosition !== "right" && props.icon}
		{props.label}
		{props.icon && props.iconPosition === "right" && props.icon}
	</button>;
};

export default Button;