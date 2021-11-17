import { useEffect, useState } from "react";
import { User } from "../models/User";

/**
 * Needs work; feels hacky having to rely on a special meaning for undefined.
 * undefined = user info being fetched
 * null = no user logged in
 * @returns
 */
export default function useUser(): User | null | undefined {
	const [user, setUser] = useState<User | null | undefined>(undefined);

	useEffect(() => {
		fetch("/api/user/info").then(response => {
			if (response.status === 200) {
				// TODO: read data from response,
				// or better: add this route to upholiService + wasm
				setUser({
					id: "_",
					username: "_"
				});
			}
			else {
				setUser(null);
			}
		}).catch(console.error);
	}, []);

	return user;
}