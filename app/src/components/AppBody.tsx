import * as React from "react";
import { Route } from "react-router-dom";
import 'react-toastify/dist/ReactToastify.css';
import LibraryPage from "../pages/LibraryPage";
import AlbumsPage from "../pages/AlbumsPage";
import CollectionsPage from "../pages/CollectionsPage";
import AlbumPage from "../pages/AlbumPage";
import PhotoPage from "../pages/PhotoPage";
import CollectionPage from "../pages/CollectionPage";
import SharedCollectionPage from "../pages/SharedCollectionPage";
import Header from "./Header";

interface AppBodyProps {

}

interface AppBodyState {
	header: {
		renderMenu: boolean,
		actions: JSX.Element | null,
		contextMenu: JSX.Element | null
	}
}

/**
 * Renders headers and route components.
 */
class AppBody extends React.Component<AppBodyProps, AppBodyState> {
	constructor(props: AppBodyProps) {
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
	updateHeader(renderNavMenu: boolean, actions: JSX.Element | null, contextMenu: JSX.Element | null) {
		this.setState({
			header: {
				renderMenu: renderNavMenu,
				actions: actions,
				contextMenu: contextMenu
			}
		});
	}

	render() {
		const fnRenderRoute = (path: string, component: any /* Correct type = ? */, requiresAuthentication: boolean) => {
			return <Route path={path} exact render={(props: any) => {
				props.onHeaderUpdated = (renderNavMenu: boolean, actions: JSX.Element | null, contextMenu: JSX.Element | null) => this.updateHeader(renderNavMenu, actions, contextMenu);
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