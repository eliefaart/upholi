import * as React from "react";
import { History } from "history";

interface AppState {
	authenticated: boolean,
	history: History | null,
	headerActions: JSX.Element | null
}

const AppStateContext: React.Context<AppState> = React.createContext<AppState>({
	authenticated: false,
	history: null,
	headerActions: null
});

export default AppStateContext;