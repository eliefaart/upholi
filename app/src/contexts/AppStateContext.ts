import * as React from "react";
import { History, createBrowserHistory } from "history";

export interface AppState {
  history: History;
}

const appStateContext: React.Context<AppState> = React.createContext<AppState>({
  history: createBrowserHistory(),
});

export default appStateContext;
