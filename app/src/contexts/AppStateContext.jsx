import React from "react";

const AppStateContext = React.createContext({
	authenticated: false,
	history: null,

	headerActions: null
});

export default AppStateContext;