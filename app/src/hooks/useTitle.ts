import { useEffect } from "react";

export function useTitle(title: string): void {
	useEffect(() => {
		const prevTitle = document.title;
		document.title = title;

		return () => {
			document.title = prevTitle;
		};
	});
}