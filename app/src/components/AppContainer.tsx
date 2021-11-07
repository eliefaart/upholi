import * as React from "react";
import { FC } from "react";
import { Router } from "react-router-dom";
import { ToastContainer, Zoom } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";
import AppBody from "./layout/AppBody";
import appStateContext from "../contexts/AppStateContext";
import useUser from "../hooks/useUser";

/**
 * Highest component in hierarchy, initializes history/router, context, modals and toast messages.
 */
const AppContainer: FC = () => {
	const user = useUser();

	// Create a new browser history object
	// Store this in a context, so any component can access it and navigate
	const context = React.useContext(appStateContext);
	context.authenticated = !!user;

	if (!context.authenticated) {
		return null;
	}
	else {
		return (
			<Router history={context.history}>
				<appStateContext.Provider value={context}>
					<AppBody/>
				</appStateContext.Provider>

				<ToastContainer position="bottom-center"
					autoClose={3000}
					hideProgressBar
					newestOnTop
					closeOnClick
					rtl={false}
					draggable
					pauseOnHover
					transition={Zoom}/>
			</Router>
		);
	}
};

export default AppContainer;