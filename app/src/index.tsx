import * as React from "react";
import * as ReactDOM from "react-dom";
import AppContainer from "./components/AppContainer";
import "./skin/app.scss";

import init, {greet, hello} from "hello-wasm";

// Render a page in container
const rootElement = document.getElementById("appRoot");
ReactDOM.render(<AppContainer/>, rootElement);

init("dist/hello_wasm.wasm").then(() => {
	console.warn(hello("eric"));
	greet("WebAssembly");
});