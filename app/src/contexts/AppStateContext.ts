import * as React from "react";
import { History, createBrowserHistory } from "history";

export interface AppState {
	authenticated: boolean,
	history: History
}

const appStateContext: React.Context<AppState> = React.createContext<AppState>({
	authenticated: false,
	history: createBrowserHistory()
});

export default appStateContext;