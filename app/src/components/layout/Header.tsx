import * as React from "react";
import { FC } from "react";
import appStateContext, { AppState } from "../../contexts/AppStateContext";
import { HeaderSettings } from "../../models/HeaderSettings";
import { IconContextMenu } from "../misc/Icons";

interface Props {
	settings: HeaderSettings
}

const Header: FC<Props> = (props) => {
	const [contextMenuOpen, setContextMenuOpen] = React.useState(false);
	const context = React.useContext<AppState>(appStateContext);

	const menuItems = [
		{ path: "/", title: "Library" },
		{ path: "/albums", title: "Albums" },
		{ path: "/shared", title: "Shared" }
	];

	const headerEmpty = !props.settings.visible && !props.settings.headerActions && !props.settings.headerContextMenu;
	if (headerEmpty) {
		return null;
	}
	else {
		return <header id="header">
			{/* Navigation menu */}
			{props.settings.visible !== false && <nav>
				{menuItems.map((menuItem) =>
				(<span key={menuItem.path}
					onClick={() => context.history.push(menuItem.path)}
					className={location.pathname === menuItem.path ? "active" : ""}
				>{menuItem.title}</span>)
				)}
			</nav>}

			{/* Empty filler space in middle */}
			<span className="buffer">&nbsp;</span>

			{/* Actions: buttons on right-side of header */}
			{!!props.settings.headerActions && <div className="actions">
				{props.settings.headerActions}
			</div>}

			{/* Additional action buttons behind a context menu button */}
			{!!props.settings.headerContextMenu && contextMenuOpen && <div className="context-menu" onClick={() => setContextMenuOpen(false)}>
				<div className="items">
					{props.settings.headerContextMenu}
				</div>
			</div>}

			{/* Context menu toggle button */}
			{!!props.settings.headerContextMenu && <button className="context-menu-toggle" onClick={() => setContextMenuOpen(!contextMenuOpen)}>
				<IconContextMenu />
			</button>}
		</header>;
	}
};

export default Header;