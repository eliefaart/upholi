import { useEffect, useState } from "react";
import { User } from "../models/User";

export default function useUser(): User | null {
	const [user, setUser] = useState<User | null>(null);

	useEffect(() => {
		fetch("/api/user/info").then(response => {
			setUser({
				id: "_",
				username: "_"
			});
		}).catch(console.error);
	}, []);

	return user;
}