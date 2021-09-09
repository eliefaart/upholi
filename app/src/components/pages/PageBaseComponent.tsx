import * as React from "react";
import appStateContext from "../../contexts/AppStateContext";

export interface PageBaseComponentProps {
	requiresAuthentication: boolean,
	renderHeaderNavMenu: boolean,
	onHeaderUpdated: (renderHeaderNavMenu: boolean, headerActions: JSX.Element | null, headerContextMenu: JSX.Element | null) => void,
	// eslint-disable-next-line  @typescript-eslint/no-explicit-any
	match: any	// TODO: Find type. This is the 'react-router' match info
}

/**
 * Base class for a 'page components'.
 * Handles notifying parent component of updates to header state.
 */
export class PageBaseComponent<TState> extends React.Component<PageBaseComponentProps, TState> {

	lastHeaderJson: string | null;

	constructor(props: PageBaseComponentProps) {
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
	getHeaderActions(): JSX.Element | null {
		return null;
	}

	/**
	 * Returns context menu content to render in header.
	 * This function is intended to be overwritten in sub classes
	 */
	getHeaderContextMenu(): JSX.Element | null {
		return null;
	}

	getTitle(): string {
		return "upholi";
	}

	componentDidMount(): void {
		if (this.props.requiresAuthentication && !this.context.authenticated) {
			this.context.history.push("/login");
		}

		this.updateAllPageElement();
	}

	componentDidUpdate(prevProps: PageBaseComponentProps, prevState: TState): void {
		this.updateAllPageElement();
		if (super.componentDidUpdate) {
			super.componentDidUpdate(prevProps, prevState);
		}
	}

	updateAllPageElement(): void {
		this.updatePageTitle();
		this.updateHeader();
	}

	/**
	 * Update title displayed in browser tab
	 */
	updatePageTitle(): void {
		const title = this.getTitle();
		document.title = title;
	}

	/**
	 * Notify parent component if the content of the header changed.
	 */
	updateHeader(): void {
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

PageBaseComponent.contextType = appStateContext;