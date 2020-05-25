import React from 'react';
import { Router, Route } from 'react-router-dom'
import { createBrowserHistory as createHistory } from "history";

import PhotosPage from '../pages/PhotosPage.jsx';
import AlbumsPage from '../pages/AlbumsPage.jsx';
import SharedPage from '../pages/SharedPage.jsx';
import AlbumPage from '../pages/AlbumPage.jsx';
import PhotoPage from '../pages/PhotoPage.jsx';

import AppStateContext from '../contexts/AppStateContext.jsx';

class AppContainer extends React.Component {

	constructor(props) {
		super(props);
	}

	componentDidMount() {
		let _this = this;
	}

	render() {
		const history = createHistory();

		this.context = {
			history: history
		}

		return (
			<Router history={history}>
				<div id="app">
					<AppStateContext.Provider value={this.context}>
						<Route path="/" exact component={PhotosPage} />
						<Route path="/photos" exact component={PhotosPage} />
						<Route path="/albums" exact component={AlbumsPage} />
						<Route path="/shared" exact component={SharedPage} />
						<Route path="/photo/:photoId" exact component={PhotoPage} />
						<Route path="/album/:albumId" exact component={AlbumPage} />
					</AppStateContext.Provider>
				</div>
			</Router>
		);
	}
}
AppContainer.contextType = AppStateContext;

export default AppContainer;