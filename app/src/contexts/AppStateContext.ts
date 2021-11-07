import * as React from "react";
import { History, createBrowserHistory } from "history";

export interface AppState {
	history: History,

	// Temp: Until I can switch to using hooks for this
	headerActions: JSX.Element | null,
	headerContextMenu: JSX.Element | null,
}

const appStateContext: React.Context<AppState> = React.createContext<AppState>({
	history: createBrowserHistory(),
	headerActions: null,
	headerContextMenu: null
});

export default appStateContext;