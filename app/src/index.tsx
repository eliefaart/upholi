import * as React from "react";
import { createRoot } from "react-dom/client";
import * as ReactModal from "react-modal";
import App from "./components/App";

import init from "wasm";
import "./skin/app.scss";

init("/dist/wasm.wasm").then(() => {
  const appRootElementId = "appRoot";
  const rootElement = document.getElementById("appRoot");
  if (rootElement) {
    ReactModal.setAppElement(rootElement);
    const root = createRoot(rootElement);
    root.render(<App />);
  } else {
    console.error(`Element with ID ${appRootElementId} not found.`);
  }
});
