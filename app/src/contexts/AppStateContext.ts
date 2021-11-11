import * as React from "react";
import { History, createBrowserHistory } from "history";
import { HeaderSettings } from "../hooks/useHeader";

export interface AppState {
	history: History,
	header: HeaderSettings
}

const appStateContext: React.Context<AppState> = React.createContext<AppState>({
	history: createBrowserHistory(),
	header: {
		visible: false
	}
});

export default appStateContext;