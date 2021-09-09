import * as React from "react";
import { Route } from "react-router-dom";
import "react-toastify/dist/ReactToastify.css";
import LibraryPage from "./pages/LibraryPage";
import AlbumsPage from "./pages/AlbumsPage";
import AlbumPage from "./pages/AlbumPage";
//import PhotoPage from "./pages/PhotoPage";
import SharedPage from "./pages/SharedPage";
import SharedCollectionPage from "./pages/SharedCollectionPage";
import Header from "./Header";
import LoginPage from "./pages/LoginPage";

interface AppBodyProps { }

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
		};
	}

	/**
	 * Handle updated header content received from child component
	 */
	updateHeader(renderNavMenu: boolean, actions: JSX.Element | null, contextMenu: JSX.Element | null): void {
		this.setState({
			header: {
				renderMenu: renderNavMenu,
				actions: actions,
				contextMenu: contextMenu
			}
		});
	}

	render(): React.ReactNode {
		// eslint-disable-next-line  @typescript-eslint/no-explicit-any
		const fnRenderRoute = (path: string, component: any, requiresAuthentication: boolean) => {
			// eslint-disable-next-line  @typescript-eslint/no-explicit-any
			return <Route path={path} exact render={(props: any) => {
				props.onHeaderUpdated = (renderNavMenu: boolean, actions: JSX.Element | null, contextMenu: JSX.Element | null) => this.updateHeader(renderNavMenu, actions, contextMenu);
				props.requiresAuthentication = requiresAuthentication;
				props.renderHeaderNavMenu = requiresAuthentication;	// Can determine this based on wether auth is required for now

				return React.createElement(component, props);
			}}/>;
		};

		return (
			<React.Fragment>
				<Header
					renderMenu={this.state.header.renderMenu}
					actions={this.state.header.actions}
					contextMenu={this.state.header.contextMenu}/>

				{fnRenderRoute("/", LibraryPage, true)}
				{fnRenderRoute("/login", LoginPage, false)}
				{fnRenderRoute("/register", LoginPage, false)}
				{fnRenderRoute("/albums", AlbumsPage, true)}
				{fnRenderRoute("/shared", SharedPage, true)}
				{fnRenderRoute("/album/:albumId", AlbumPage, true)}
				{fnRenderRoute("/s/:token", SharedCollectionPage, false)}

				{/* <ModalUploadProgress/> */}
			</React.Fragment>
		);
	}
}

export default AppBody;