import * as React from "react";
import { FC } from "react";
import { Router } from "react-router-dom";
import { ToastContainer, Zoom } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";
import Layout from "./layout/Layout";
import appStateContext from "../contexts/AppStateContext";
import useUser from "../hooks/useUser";


import { Route } from "react-router-dom";
import "react-toastify/dist/ReactToastify.css";
import LibraryPage from "./pages/LibraryPage";
import AlbumsPage from "./pages/AlbumsPage";
import AlbumPage from "./pages/AlbumPage";
import SharedPage from "./pages/SharedPage";
import SharedAlbumPage from "./pages/SharedAlbumPage";
import LoginPage from "./pages/LoginPage";
import Header from "./layout/Header";
import UploadProgress from "./misc/UploadProgress";
import Authentication from "./layout/Authentication";

interface State {
	header: {
		renderMenu: boolean,
		actions: JSX.Element | null,
		contextMenu: JSX.Element | null
	}
}

/**
 * Highest component in hierarchy, initializes history/router, context, modals and toast messages.
 */
const App: FC = () => {
	const user = useUser();
	const [state, setState] = React.useState<State>({
		header: {
			renderMenu: false,
			actions: null,
			contextMenu: null
		}
	});

	// Create a new browser history object
	// Store this in a context, so any component can access it and navigate
	const context = React.useContext(appStateContext);
	// context.authenticated = !!user;

	// const updateHeader = (renderNavMenu: boolean, actions: JSX.Element | null, contextMenu: JSX.Element | null): void => {
	// 	setState({
	// 		header: {
	// 			renderMenu: renderNavMenu,
	// 			actions: actions,
	// 			contextMenu: contextMenu
	// 		}
	// 	});
	// };

	const fnRenderRoute = (path: string, component: any, requiresAuthentication: boolean) => {
		// eslint-disable-next-line  @typescript-eslint/no-explicit-any
		return <Route path={path} exact render={(props: any) => {
			//props.onHeaderUpdated = updateHeader;
			props.renderHeaderNavMenu = requiresAuthentication;	// Can determine this based on wether auth is required for now

			return <Authentication requiresAuthentication={requiresAuthentication}>
				{React.createElement(component, props)}
			</Authentication>;
		}}/>;
	};

	return <Router history={context.history}>
		<appStateContext.Provider value={context}>
			<Layout>
				{fnRenderRoute("/", LibraryPage, true)}
				{fnRenderRoute("/login", LoginPage, false)}
				{fnRenderRoute("/register", LoginPage, false)}
				{fnRenderRoute("/albums", AlbumsPage, true)}
				{fnRenderRoute("/shared", SharedPage, true)}
				{fnRenderRoute("/album/:albumId", AlbumPage, true)}
				{fnRenderRoute("/s/:token", SharedAlbumPage, false)}
			</Layout>
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
	</Router>;
};

export default App;