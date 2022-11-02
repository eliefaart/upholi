import axios from "axios";
import { useEffect, useState } from "react";

type resetAuthenticationStatus = () => void;

export enum AuthenticationStatus {
	// Status hasn't been determined yet.
	Unknown,
	// Status is currently being fetched from server.
	Refreshing,
	// Client is an authenticated user.
	Authenticated,
	// Client is not an authenticated user, anonymous client.
	Unauthenticad,
}

let lastStatus: AuthenticationStatus = AuthenticationStatus.Unknown;

export default function useAuthenticationStatus(): [AuthenticationStatus, resetAuthenticationStatus] {
	const [state, setState] = useState<AuthenticationStatus>(lastStatus);

	const resetStatus = () => setState(AuthenticationStatus.Unknown);

	const refresh = () => {
		setState(AuthenticationStatus.Refreshing);
		axios.get("/user")
			.then(() => setState(AuthenticationStatus.Authenticated))
			.catch(() => setState(AuthenticationStatus.Unauthenticad));
	};

	useEffect(() => {
		if (state === AuthenticationStatus.Unknown) {
			refresh();
		}
	}, []);

	// Keep track of last known status, so we don't need to redetermine if client navigates to another page
	useEffect(() => {
		lastStatus = state;
	}, [state]);

	return [state, resetStatus];
}