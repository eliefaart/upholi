import React from 'react';
import { Router, Route } from 'react-router-dom';
import { createBrowserHistory as createHistory } from "history";
import { ToastContainer, Zoom } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';
import PhotosPage from '../pages/PhotosPage.jsx';
import AlbumsPage from '../pages/AlbumsPage.jsx';
import SharedPage from '../pages/SharedPage.jsx';
import AlbumPage from '../pages/AlbumPage.jsx';
import PhotoPage from '../pages/PhotoPage.jsx';
import SharedCollectionPage from '../pages/SharedCollectionPage.jsx';
import SharedCollectionPhotoPage from '../pages/SharedCollectionPhotoPage.jsx';

import AppStateContext from '../contexts/AppStateContext.jsx';

class AppContainer extends React.Component {

	constructor(props) {
		super(props);
	}
	
	render() {
		// Create a new browser history object
		// Store this in a context, that any component can access to add navigate
		const history = createHistory();
		this.context = {
			history: history
		}

		return (
			<Router history={history}>
				<div id="app">
					<AppStateContext.Provider value={this.context}>
						<Route path="/" exact component={PhotosPage} />
						{/* <Route path="/photos" exact component={PhotosPage} /> */}
						<Route path="/albums" exact component={AlbumsPage} />
						<Route path="/shared" exact component={SharedPage} />
						<Route path="/photo/:photoId" exact component={PhotoPage} />
						<Route path="/album/:albumId" exact component={AlbumPage} />
						<Route path="/shared/collection/:collectionId" exact component={SharedCollectionPage} />
						<Route path="/shared/collection/:collectionId/photo/:photoId" exact component={SharedCollectionPhotoPage} />
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