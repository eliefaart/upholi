import axios from "axios";
import { useEffect, useState } from "react";
import { User } from "../models/User";

export enum AuthenticationStatus {
	// Hasn't been determined yet.
	Unknown,
	// Client is an authenticated user.
	Authenticated,
	// Client is not an authenticated user, anonymous client.
	Unauthenticad,
}

let lastStatus: AuthenticationStatus = AuthenticationStatus.Unknown;

export default function useAuthenticationStatus(): AuthenticationStatus {
	const [state, setState] = useState<AuthenticationStatus>(lastStatus);

	useEffect(() => {
		if (!state) {
			axios.get<User>("/api/user/info")
				.then(() => setState(AuthenticationStatus.Authenticated))
				.catch(() => setState(AuthenticationStatus.Unauthenticad));
		}
	}, []);

	useEffect(() => {
		// Keep track of last known status, so we don't need to redetermine if client navigates to another page
		lastStatus = state;
	}, [state]);

	return state;
}