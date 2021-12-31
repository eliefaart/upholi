import * as React from "react";
import { FC } from "react";
import appStateContext, { AppState } from "../../contexts/AppStateContext";
import { IconContextMenu } from "../misc/Icons";

interface Props {
	headerActions?: JSX.Element | null,
	headerContextMenu?: JSX.Element | null,
}

const DefaultHeaderContent: FC<Props> = (props) => {
	const [contextMenuOpen, setContextMenuOpen] = React.useState(false);
	const context = React.useContext<AppState>(appStateContext);

	const menuItems = [
		{ path: "/", title: "Library" },
		{ path: "/albums", title: "Albums" },
		{ path: "/shared", title: "Shared" }
	];

	return <>
		{/* Navigation menu */}
		<nav>
			{menuItems.map((menuItem) =>
			(<span key={menuItem.path}
				onClick={() => context.history.push(menuItem.path)}
				className={location.pathname === menuItem.path ? "active" : ""}
			>{menuItem.title}</span>)
			)}
		</nav>

		{/* Empty filler space in middle */}
		<span className="buffer">&nbsp;</span>

		{/* Actions: buttons on right-side of header */}
		{!!props.headerActions && <div className="actions">
			{props.headerActions}
		</div>}

		{/* Additional action buttons behind a context menu button */}
		{!!props.headerContextMenu && contextMenuOpen && <div className="context-menu" onClick={() => setContextMenuOpen(false)}>
			<div className="items">
				{props.headerContextMenu}
			</div>
		</div>}

		{/* Context menu toggle button */}
		{!!props.headerContextMenu && <button className="context-menu-toggle" onClick={() => setContextMenuOpen(!contextMenuOpen)}>
			<IconContextMenu />
		</button>}
	</>;
};

export default DefaultHeaderContent;