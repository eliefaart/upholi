import * as React from "react";
import { History } from "history";

export interface AppState {
	authenticated: boolean,
	history: History | null,
	headerActions: JSX.Element | null,

}

const appStateContext: React.Context<AppState> = React.createContext<AppState>({
	authenticated: false,
	history: null,
	headerActions: null
});

export default appStateContext;