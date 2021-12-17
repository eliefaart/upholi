import * as React from "react";
import { FC } from "react";

interface Props {
	errors: string[]
}

const Errors: FC<Props> = (props) => {
	if (props.errors.length === 0) {
		return <></>;
	}
	else {
		return <ul className="errors">
			{props.errors.map(error => <li key={error}>{error}</li>)}
		</ul>;
	}
};

export default Errors;