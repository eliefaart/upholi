import * as React from "react";
import { FC } from "react";
import appStateContext from "../../contexts/AppStateContext";
import useUser from "../../hooks/useUser";

interface Props {
	requiresAuthentication: boolean
}

const Authentication: FC<Props> = (props) => {
	const user = useUser();
	const context = React.useContext(appStateContext);

	const authenticated = !!user;

	if (user === undefined) {
		// User info is still being fetched.
		// TODO: Make it nicer than having to check undefined.
		return <></>;
	}
	else if (!authenticated && props.requiresAuthentication) {
		context.history.push("/login");
		return <></>;
	}
	else {
		return <>{props.children}</>;
	}
};

export default Authentication;