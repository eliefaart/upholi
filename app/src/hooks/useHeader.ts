import { useState } from "react";

interface HeaderSettings {
	visible: boolean,
	headerActions?: JSX.Element | null,
	headerContextMenu?: JSX.Element | null
}

let updateHeaderState: React.Dispatch<React.SetStateAction<HeaderSettings>> | null = null;

export function useHeader(): HeaderSettings {
	const [state, setState] = useState<HeaderSettings>({
		visible: false,
		headerActions: null,
		headerContextMenu: null
	});

	updateHeaderState = setState;

	return state;
}

export function setHeader(settings: HeaderSettings): void {
	updateHeaderState && updateHeaderState(settings);
}