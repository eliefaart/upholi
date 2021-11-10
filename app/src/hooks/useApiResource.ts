import { useEffect, useState } from "react";

export function useApiResource<T>(fetch: () => Promise<T>, initialValue: T): [T, () => void] {
	const [state, setState] = useState<T>(initialValue);

	const refresh = () => {
		fetch().then(setState).catch(console.error);
	};

	useEffect(() => {
		refresh();
	}, []);

	return [state, refresh];
}