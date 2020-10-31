import React from "react";
import { default as ReactModal } from "react-modal";
import Header from "./Header.jsx";
import AppStateContext from "../contexts/AppStateContext.jsx";

class PageLayout extends React.Component {

	constructor(props) {
		super(props);

		this.state = {
			authorized: false
		};
	}

	componentDidMount() {
		ReactModal.setAppElement("#app");
	}

	render() {
		if (this.props.requiresAuthentication && !this.context.authenticated) {
			// TODO: This is a temporary implementation,
			// should redirect to a Welcome component or something,
			// which would have a login button that starts the login flow
			document.location = "/oauth/start";
		}

		// Change document title
		let pageTitle = this.props.title || "Hummingbird";
		document.title = pageTitle;

		const headerVisible = this.props.renderMenu
			|| !!this.props.headerActions
			|| !!this.props.headerContextMenuActions;

		return (
			<div className={"page " + (headerVisible ? "hasHeader" : "")}
				onDrop={this.props.onDrop}
				onDragOver={this.props.onDragOver || ((event) => event.preventDefault())}>
				{headerVisible && <Header
					renderMenu={this.props.renderMenu}
					actionsElement={this.props.headerActions}
					contextMenuElement={this.props.headerContextMenuActions}
					>
				</Header>}

				<div className="content">
					{this.props.children}
				</div>
			</div>
		);
	}
}

PageLayout.contextType = AppStateContext;
export default PageLayout;