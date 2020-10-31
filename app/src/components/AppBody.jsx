import React from "react";
import { Route } from "react-router-dom";
import 'react-toastify/dist/ReactToastify.css';
import LibraryPage from "../pages/LibraryPage.jsx";
import AlbumsPage from "../pages/AlbumsPage.jsx";
import CollectionsPage from "../pages/CollectionsPage.jsx";
import AlbumPage from "../pages/AlbumPage.jsx";
import PhotoPage from "../pages/PhotoPage.jsx";
import CollectionPage from "../pages/CollectionPage.jsx";
import SharedCollectionPage from "../pages/SharedCollectionPage.jsx";
import Header from "./Header.jsx";

/**
 * Renders headers and route components.
 */
class AppBody extends React.Component {
	constructor(props) {
		super(props);

		this.state = {
			header: {
				renderMenu: false,
				actions: null,
				contextMenu: null
			}
		}
	}

	/**
	 * Handle updated header content received from child component
	 */
	updateHeader(renderNavMenu, actions, contextMenu) {
		this.setState({
			header: {
				renderMenu: renderNavMenu,
				actions: actions,
				contextMenu: contextMenu
			}
		});
	}

	render() {
		const fnRenderRoute = (path, component, requiresAuthentication) => {
			return <Route path={path} exact render={props => {
				props.onHeaderUpdated = (renderNavMenu, actions, contextMenu) => this.updateHeader(renderNavMenu, actions, contextMenu);
				props.requiresAuthentication = requiresAuthentication;
				props.renderHeaderNavMenu = requiresAuthentication;	// Can determine this based on wether auth is required for now

				return React.createElement(component, props);
			}}/>
		};

		return (
			<React.Fragment>
				<Header
					renderMenu={this.state.header.renderMenu}
					actions={this.state.header.actions}
					contextMenu={this.state.header.contextMenu}/>

				{fnRenderRoute("/", LibraryPage, true)}
				{fnRenderRoute("/albums", AlbumsPage, true)}
				{fnRenderRoute("/collections", CollectionsPage, true)}
				{fnRenderRoute("/collection/:collectionId", CollectionPage, true)}
				{fnRenderRoute("/photo/:photoId", PhotoPage, false)}
				{fnRenderRoute("/album/:albumId", AlbumPage, true)}
				{fnRenderRoute("/shared/collection/:token", SharedCollectionPage, false)}
			</React.Fragment>
		);
	}
}

export default AppBody;