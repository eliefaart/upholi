import React from "react";
import { Router, Route } from "react-router-dom";
import { createBrowserHistory as createHistory } from "history";
import { ToastContainer, Zoom } from "react-toastify";
import 'react-toastify/dist/ReactToastify.css';
import PhotosPage from "../pages/PhotosPage.jsx";
import AlbumsPage from "../pages/AlbumsPage.jsx";
import CollectionsPage from "../pages/CollectionsPage.jsx";
import AlbumPage from "../pages/AlbumPage.jsx";
import PhotoPage from "../pages/PhotoPage.jsx";
import CollectionPage from "../pages/CollectionPage.jsx";
import SharedCollectionPage from "../pages/SharedCollectionPage.jsx";

import AppStateContext from "../contexts/AppStateContext.jsx";

class AppContainer extends React.Component {

	constructor(props) {
		super(props);
		
		this.context = React.createContext();
		this.state = {
			ready: false
		}
	}

	componentDidMount() {
		// Call server to find out if user is authenticated
		fetch("/oauth/user/info").then((response) => {
			this.context.authenticated = response.status === 200;
			this.setState({
				ready: true
			});
		}).catch(console.error)
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
				<div id="app">
					<AppStateContext.Provider value={this.context}>
						<Route path="/" exact component={PhotosPage} />
						<Route path="/albums" exact component={AlbumsPage} />
						<Route path="/collections" exact component={CollectionsPage} />
						<Route path="/collection/:collectionId" exact component={CollectionPage} />
						<Route path="/photo/:photoId" exact component={PhotoPage} />
						<Route path="/album/:albumId" exact component={AlbumPage} />
						<Route path="/shared/collection/:collectionId" exact component={SharedCollectionPage} />
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
				</div>
			</Router>
		);
	}
}

AppContainer.contextType = AppStateContext;
export default AppContainer;