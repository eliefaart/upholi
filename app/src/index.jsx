import React from "react";
import ReactDOM from "react-dom";
import AppContainer from "./components/AppContainer.jsx";
import './skin/app.scss';

// Render a page in container
const rootElement = document.getElementById('appRoot');
ReactDOM.render(<AppContainer/>, rootElement);