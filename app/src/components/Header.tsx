import * as React from "react";
import appStateContext from "../contexts/AppStateContext";
import { IconContextMenu } from "./misc/Icons";

interface HeaderProps {
	renderMenu: boolean,
	actions: JSX.Element | null,
	contextMenu: JSX.Element | null
}

interface HeaderState {
	contextMenuOpen: boolean
}

class Header extends React.Component<HeaderProps, HeaderState> {

	constructor(props: HeaderProps) {
		super(props);

		this.state = {
			contextMenuOpen: false
		};
	}

	render(): React.ReactNode {
		const headerEmpty = !this.props.renderMenu && !this.props.actions && !this.props.contextMenu;
		if (headerEmpty)
			return null;

		const gotoPage = (path: string) => {
			!!this.context.history && this.context.history.push(path);
		};
		const menuItems = [
			{ path: "/", title: "Library" },
			{ path: "/albums", title: "Albums" },
			{ path: "/shared", title: "Shared" }
		];

		return <header id="header">
			{/* Navigation menu */}
			{this.props.renderMenu !== false && <nav>
				{menuItems.map((menuItem) =>
					(<span key={menuItem.path}
						onClick={() => gotoPage(menuItem.path)}
						className={location.pathname === menuItem.path ? "active" : ""}
						>{menuItem.title}</span>)
				)}
			</nav>}

			{/* Empty filler space in middle */}
			<span className="buffer">&nbsp;</span>

			{/* Actions: buttons on right-side of header */}
			{!!this.props.actions && <div className="actions">
				{this.props.actions}
			</div>}

			{/* Additional action buttons behind a context menu button */}
			{!!this.props.contextMenu && this.state.contextMenuOpen && <div className="contextMenu" onClick={() => this.setState({contextMenuOpen: false})}>
				<div className="items">
					{this.props.contextMenu}
				</div>
			</div>}

			{/* Context menu toggle button */}
			{!!this.props.contextMenu && <button className="contextMenuToggle" onClick={() => this.setState({contextMenuOpen: !this.state.contextMenuOpen})}>
				<IconContextMenu/>
			</button>}
		</header>;
	}
}

Header.contextType = appStateContext;
export default Header;