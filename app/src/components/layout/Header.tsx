import * as React from "react";
import { FC } from "react";
import appStateContext, { AppState } from "../../contexts/AppStateContext";
import { useHeader } from "../../hooks/useHeader";
import { IconContextMenu } from "../misc/Icons";

interface Props {
	// renderMenu: boolean,
	// actions: JSX.Element | null,
	// contextMenu: JSX.Element | null
}

const Header: FC<Props> = (props) => {
	const header = useHeader();
	const [contextMenuOpen, setContextMenuOpen] = React.useState(false);
	const { history } = React.useContext<AppState>(appStateContext);

	const menuItems = [
		{ path: "/", title: "Library" },
		{ path: "/albums", title: "Albums" },
		{ path: "/shared", title: "Shared" }
	];

	const headerEmpty = !header.visible && !header.headerActions && !header.headerContextMenu;
	if (headerEmpty) {
		return null;
	}
	else {
		return <header id="header">
			{/* Navigation menu */}
			{header.visible !== false && <nav>
				{menuItems.map((menuItem) =>
					(<span key={menuItem.path}
						onClick={() => history.push(menuItem.path)}
						className={location.pathname === menuItem.path ? "active" : ""}
						>{menuItem.title}</span>)
				)}
			</nav>}

			{/* Empty filler space in middle */}
			<span className="buffer">&nbsp;</span>

			{/* Actions: buttons on right-side of header */}
			{!!header.headerActions && <div className="actions">
				{header.headerActions}
			</div>}

			{/* Additional action buttons behind a context menu button */}
			{!!header.headerContextMenu && contextMenuOpen && <div className="contextMenu" onClick={() => setContextMenuOpen(false)}>
				<div className="items">
					{header.headerContextMenu}
				</div>
			</div>}

			{/* Context menu toggle button */}
			{!!header.headerContextMenu && <button className="contextMenuToggle" onClick={() => setContextMenuOpen(!contextMenuOpen)}>
				<IconContextMenu/>
			</button>}
		</header>;
	}
};

export default Header;