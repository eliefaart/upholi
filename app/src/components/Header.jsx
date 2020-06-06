import React from 'react';

import AppStateContext from '../contexts/AppStateContext.jsx';

class Header extends React.Component {

	constructor(props) {
		super(props);
	}
	
	componentDidMount() {
	}
	
	componentWillUnmount() {
	}

	gotoPhotos() {
		!!this.context.history && this.context.history.push("/");
	}

	gotoAlbums() {
		!!this.context.history && this.context.history.push("/albums");
	}

	gotoShared() {
		!!this.context.history && this.context.history.push("/shared");
	}
	
	render() {
		const gotoPage = (path) => {
			!!this.context.history && this.context.history.push(path);
		}
		let menuItems = [
			{ path: "/", title: "Home" },
			{ path: "/albums", title: "Albums" },
			{ path: "/shared", title: "Shared" }
		];

		let menuItemsElement = menuItems.map((menuItem) => {
			return (<span key={menuItem.path} onClick={() => gotoPage(menuItem.path)}>{menuItem.title}</span>);
		});

		return (
			<div className="header">
				{this.props.renderMenu !== false && <div className="menu">
					{menuItems.map((menuItem) => 
						(<span key={menuItem.path} 
							onClick={() => gotoPage(menuItem.path)}
							className={location.pathname === menuItem.path ? "active" : ""}
							>{menuItem.title}</span>)
					)}
				</div>}

				<span className="title">{this.props.title || ' '}</span>
				<div className="actions">
					{this.props.children}
				</div>
			</div>
		);
	}
}

Header.contextType = AppStateContext;
export default Header;