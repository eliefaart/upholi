import * as React from "react";
import { FC } from "react";
import appStateContext, { AppState } from "../../contexts/AppStateContext";
import { IconContextMenu } from "../misc/Icons";

interface Props {
	renderMenu: boolean,
	actions: JSX.Element | null,
	contextMenu: JSX.Element | null
}

const Header: FC<Props> = (props) => {
	const [contextMenuOpen, setContextMenuOpen] = React.useState(false);
	const { history } = React.useContext<AppState>(appStateContext);

	const menuItems = [
		{ path: "/", title: "Library" },
		{ path: "/albums", title: "Albums" },
		{ path: "/shared", title: "Shared" }
	];
	const gotoPage = (path: string) => !!history && history.push(path);

	const headerEmpty = !props.renderMenu && !props.actions && !props.contextMenu;
	if (headerEmpty) {
		return null;
	}
	else {
		return <header id="header">
			{/* Navigation menu */}
			{props.renderMenu !== false && <nav>
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
			{!!props.actions && <div className="actions">
				{props.actions}
			</div>}

			{/* Additional action buttons behind a context menu button */}
			{!!props.contextMenu && contextMenuOpen && <div className="contextMenu" onClick={() => setContextMenuOpen(false)}>
				<div className="items">
					{props.contextMenu}
				</div>
			</div>}

			{/* Context menu toggle button */}
			{!!props.contextMenu && <button className="contextMenuToggle" onClick={() => setContextMenuOpen(!contextMenuOpen)}>
				<IconContextMenu/>
			</button>}
		</header>;
	}
};

export default Header;