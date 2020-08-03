import React from "react";

const AppStateContext = React.createContext({
	authenticated: false,
	history: null
});

export default AppStateContext;