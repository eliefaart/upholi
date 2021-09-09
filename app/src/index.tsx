import * as React from "react";
import * as ReactDOM from "react-dom";
import AppContainer from "./components/AppContainer";
import "./skin/app.scss";

import init from "wasm";

init("/dist/wasm.wasm").then(() => {
	// Render a page in container
	const rootElement = document.getElementById("appRoot");
	ReactDOM.render(<AppContainer/>, rootElement);
});