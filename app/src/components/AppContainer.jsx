import React from "react";
import { Router } from "react-router-dom";
import { createBrowserHistory as createHistory } from "history";
import { ToastContainer, Zoom } from "react-toastify";
import 'react-toastify/dist/ReactToastify.css';
import AppBody from "../components/AppBody.jsx";
import { default as ReactModal } from "react-modal";

import AppStateContext from "../contexts/AppStateContext.jsx";

/**
 * Highest component in hierarchy, initializes history/router, context, modals and toast messages.
 */
class AppContainer extends React.Component {

	constructor(props) {
		super(props);

		this.context = React.createContext();
		this.state = {
			ready: false
		}
	}

	componentDidMount() {
		ReactModal.setAppElement("#appRoot");

		// Call server to find out if user is authenticated
		fetch("/oauth/user/info").then((response) => {
			this.context.authenticated = response.status === 200;
			this.setState({
				ready: true
			});
		}).catch(console.error);
	}

	render() {
		if (!this.state.ready)
			return null;

		// Create a new browser history object
		// Store this in a context, so any component can access it and navigate
		const history = createHistory();
		this.context.history = history;

		return (
			<Router history={this.context.history}>
				<AppStateContext.Provider value={this.context}>
					<AppBody/>
				</AppStateContext.Provider>

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
}

AppContainer.contextType = AppStateContext;
export default AppContainer;