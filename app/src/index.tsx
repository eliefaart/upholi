import * as React from "react";
import * as ReactDOM from "react-dom";
import * as ReactModal from "react-modal";
import AppContainer from "./components/AppContainer";

import init from "wasm";
import "./skin/app.scss";

init("/dist/wasm.wasm").then(() => {
	const rootElement = document.getElementById("appRoot");
	if (rootElement) {
		ReactModal.setAppElement(rootElement);
	}

	ReactDOM.render(<AppContainer/>, rootElement);
});