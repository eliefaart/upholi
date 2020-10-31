import React from "react";

import AppStateContext from "../contexts/AppStateContext.jsx";
import { IconContextMenu } from "../components/Icons.jsx";

class Header extends React.Component {

	constructor(props) {
		super(props);

		this.state = {
			contextMenuOpen: false
		};
	}

	render() {
		const headerEmpty = !this.props.renderMenu && !this.props.actionsElement && !this.props.contextMenuElement;
		if (headerEmpty)
			return null;

		const gotoPage = (path) => {
			!!this.context.history && this.context.history.push(path);
		}
		const menuItems = [
			{ path: "/", title: "Library" },
			{ path: "/albums", title: "Albums" },
			{ path: "/collections", title: "Collections" }
		];

		return (
			<div id="header">
				{this.props.renderMenu !== false && <div className="menu">
					{menuItems.map((menuItem) =>
						(<span key={menuItem.path}
							onClick={() => gotoPage(menuItem.path)}
							className={location.pathname === menuItem.path ? "active" : ""}
							>{menuItem.title}</span>)
					)}
				</div>}

				<span className="buffer">&nbsp;</span>

				{!!this.props.actionsElement && <div className="actions">
					{this.props.actionsElement}
				</div>}

				{!!this.props.contextMenuElement && this.state.contextMenuOpen && <div className="contextMenu" onClick={() => this.setState({contextMenuOpen: false})}>
					<div className="items">
						{this.props.contextMenuElement}
					</div>
				</div>}

				{!!this.props.contextMenuElement && <button className="contextMenuToggle" onClick={() => this.setState({contextMenuOpen: !this.state.contextMenuOpen})}>
					<IconContextMenu/>
				</button>}
			</div>
		);
	}
}

Header.contextType = AppStateContext;
export default Header;