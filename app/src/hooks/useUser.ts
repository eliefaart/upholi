import axios from "axios";
import { useEffect, useState } from "react";
import { User } from "../models/User";

type UserStatus = User | null | undefined;

let cache: UserStatus = undefined;

/**
 * Needs work; feels hacky having to rely on a special meaning for undefined.
 * undefined = user info being fetched
 * null = no user logged in
 * @returns
 */
export default function useUser(): UserStatus {
	const [user, setUser] = useState<UserStatus>(cache);

	useEffect(() => {
		if (!user) {
			axios.get<User>("/api/user/info")
				.then(response => {
					cache = response.data;
					setUser(response.data);
				}).catch(() => setUser(null));
		}
	}, []);

	return user;
}