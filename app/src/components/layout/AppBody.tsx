import * as React from "react";
import { FC } from "react";
import { Route } from "react-router-dom";
import "react-toastify/dist/ReactToastify.css";
import LibraryPage from "../pages/LibraryPage";
import AlbumsPage from "../pages/AlbumsPage";
import AlbumPage from "../pages/AlbumPage";
import SharedPage from "../pages/SharedPage";
import SharedAlbumPage from "../pages/SharedAlbumPage";
import Header from "./Header";
import LoginPage from "../pages/LoginPage";
import UploadProgress from "../misc/UploadProgress";

interface State {
	header: {
		renderMenu: boolean,
		actions: JSX.Element | null,
		contextMenu: JSX.Element | null
	}
}

/**
 * Renders headers and route components.
 */
const AppBody: FC = () => {
	const [state, setState] = React.useState<State>({
		header: {
			renderMenu: false,
			actions: null,
			contextMenu: null
		}
	});

	const updateHeader = (renderNavMenu: boolean, actions: JSX.Element | null, contextMenu: JSX.Element | null): void => {
		setState({
			header: {
				renderMenu: renderNavMenu,
				actions: actions,
				contextMenu: contextMenu
			}
		});
	};

	// eslint-disable-next-line  @typescript-eslint/no-explicit-any
	const fnRenderRoute = (path: string, component: any, requiresAuthentication: boolean) => {
		// eslint-disable-next-line  @typescript-eslint/no-explicit-any
		return <Route path={path} exact render={(props: any) => {
			props.onHeaderUpdated = updateHeader;
			props.requiresAuthentication = requiresAuthentication;
			props.renderHeaderNavMenu = requiresAuthentication;	// Can determine this based on wether auth is required for now

			return React.createElement(component, props);
		}}/>;
	};

	return (
		<React.Fragment>
			<Header
				renderMenu={state.header.renderMenu}
				actions={state.header.actions}
				contextMenu={state.header.contextMenu}/>

			{fnRenderRoute("/", LibraryPage, true)}
			{fnRenderRoute("/login", LoginPage, false)}
			{fnRenderRoute("/register", LoginPage, false)}
			{fnRenderRoute("/albums", AlbumsPage, true)}
			{fnRenderRoute("/shared", SharedPage, true)}
			{fnRenderRoute("/album/:albumId", AlbumPage, true)}
			{fnRenderRoute("/s/:token", SharedAlbumPage, false)}

			<UploadProgress/>
		</React.Fragment>
	);
};

export default AppBody;