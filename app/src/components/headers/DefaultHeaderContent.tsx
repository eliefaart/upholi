import * as React from "react";
import { FC } from "react";
import appStateContext, { AppState } from "../../contexts/AppStateContext";
import { IconContextMenu } from "../misc/Icons";

interface Props {
	actions?: JSX.Element | null,
	contextMenu?: JSX.Element | null,
}

const DefaultHeaderContent: FC<Props> = (props) => {
	const [contextMenuOpen, setContextMenuOpen] = React.useState(false);
	const context = React.useContext<AppState>(appStateContext);

	const menuItems = [
		{ path: "/", title: "Library", childPaths: [] },
		{ path: "/albums", title: "Albums", childPaths: ["/album"] },
		{ path: "/shared", title: "Shared", childPaths: [] },
	];

	return <>
		{/* Navigation menu */}
		<nav>
			{menuItems.map((menuItem) => {
				const isActive = location.pathname === menuItem.path
					|| menuItem.childPaths.some(p => location.pathname.startsWith(p));
				return <span key={menuItem.path}
					onClick={() => context.history.push(menuItem.path)}
					className={isActive ? "active" : ""}>
					{menuItem.title}
				</span>;
			})}
		</nav>

		{/* Empty filler space in middle */}
		<span className="buffer">&nbsp;</span>

		{/* Actions: buttons on right-side of header */}
		{!!props.actions && <div className="actions">
			{props.actions}
		</div>}

		{/* Additional action buttons behind a context menu button */}
		{!!props.contextMenu && contextMenuOpen && <div className="context-menu" onClick={() => setContextMenuOpen(false)}>
			<div className="items">
				{props.contextMenu}
			</div>
		</div>}

		{/* Context menu toggle button */}
		{!!props.contextMenu && <button className="context-menu-toggle" onClick={() => setContextMenuOpen(!contextMenuOpen)}>
			<IconContextMenu />
		</button>}
	</>;
};

export default DefaultHeaderContent;