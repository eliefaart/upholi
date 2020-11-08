import React from "react";
import AppStateContext from "../contexts/AppStateContext.ts";

/**
 * Base class for a 'page components'.
 * Handles notifying parent component of updates to header state.
 */
class PageBaseComponent extends React.Component {

	constructor(props) {
		super(props);

		// Contains the json string of the last header content provided to props.onHeaderUpdated
		// Used to determine if anything has changed, and if props.onHeaderUpdated needs to be called again.
		// TODO: Store a short hash instead of entire json string
		this.lastHeaderJson = null;
	}

	/**
	 * Returns all actions to render in header.
	 * This function is intended to be overwritten in sub classes
	 */
	getHeaderActions() {
		return null;
	}

	/**
	 * Returns context menu content to render in header.
	 * This function is intended to be overwritten in sub classes
	 */
	getHeaderContextMenu() {
		return null;
	}

	getTitle() {
		return "Upholi";
	}

	componentDidMount() {
		if (this.props.requiresAuthentication && !this.context.authenticated) {
			// TODO: This is a temporary implementation,
			// should redirect to a Welcome component or something,
			// which would have a login button that starts the login flow
			document.location = "/oauth/start";
		}

		this.updateAllPageElement();
	}

	componentDidUpdate() {
		this.updateAllPageElement();
	}

	updateAllPageElement() {
		this.updatePageTitle();
		this.updateHeader();
	}

	/**
	 * Update title displayed in browser tab
	 */
	updatePageTitle() {
		const title = this.getTitle();
		document.title = title;
	}

	/**
	 * Notify parent component if the content of the header changed.
	 */
	updateHeader() {
		if (this.props.onHeaderUpdated) {
			const headerActions = this.getHeaderActions();
			const headerActionsJson = JSON.stringify(headerActions);

			const headerContextMenu = this.getHeaderContextMenu();
			const headerContextMenuJson = JSON.stringify(headerContextMenu);

			const headerJson = headerActionsJson + headerContextMenuJson;

			if (this.lastHeaderJson !== headerJson) {
				this.props.onHeaderUpdated(this.props.renderHeaderNavMenu, headerActions, headerContextMenu);
				this.lastHeaderJson = headerJson;
			}
		}
	}
}

PageBaseComponent.contextType = AppStateContext;
export default PageBaseComponent;