import React from "react";
import { default as ReactModal } from "react-modal";
import Header from "./Header.jsx";

class PageLayout extends React.Component {

	constructor(props) {
		super(props);

		this.state = {
			authorized: false
		};
	}

	componentDidMount() {
		ReactModal.setAppElement("#app");

		if (this.props.requiresAuthentication) {
			let setAuthorized = () => this.setState({authorized: true});
			let startLogin = () => document.location = "/oauth/start";

			// Call server to find out if user is authorized
			// TODO: This is a temporary implementation, should redirect to a Welcome component or something, 
			// which would have a login button that starts the login flow
			fetch("/oauth/user/info").then((response) => {
				if (response.status == 200) {
					setAuthorized();
				} else {
					startLogin();
				}
			}).catch(console.error)
		}
	}

	render() {
		if (this.props.requiresAuthentication && !this.state.authorized)
			return null;

		const headerVisible = !!this.props.title
			|| this.props.renderMenu
			|| !!this.props.headerActions
			|| !!this.props.headerContextMenuActions;

		return (
			<div className={"page " + (headerVisible ? "hasHeader" : "")}
				onDrop={this.props.onDrop} 
				onDragOver={this.props.onDragOver || ((event) => event.preventDefault())}>
				{headerVisible && <Header title={this.props.title} 
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

export default PageLayout;