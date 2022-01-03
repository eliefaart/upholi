import * as React from "react";
import { FC } from "react";
import ReactSwitch from "react-switch";

interface Props {
	label: string,
	checked: boolean,
	onChange: (checked: boolean) => void
}

const Switch: FC<Props> = (props) => {
	return <label className="switch">
		<ReactSwitch checked={props.checked}
			width={40}
			height={15}
			handleDiameter={25}
			onColor="#30e2b3"
			offColor="#1c1c1c"
			checkedIcon={<span className="icon-checked" />}
			uncheckedIcon={<span className="icon-unchecked" />}
			onChange={props.onChange}
		/>
		<span>{props.label}</span>
	</label>;
};

export default Switch;