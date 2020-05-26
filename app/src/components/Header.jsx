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
		return (
			<div className="header">
				<div className="menu">
					<span onClick={() => this.gotoPhotos()}>Home</span>
					<span onClick={() => this.gotoAlbums()}>Albums</span>
					<span onClick={() => this.gotoShared()}>Shared</span>
				</div>
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