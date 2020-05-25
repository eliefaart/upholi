import React from 'react';

class Overlay extends React.Component {

	constructor(props) {
		super(props);
	}

	onClick(e) {
		this.props.onClick(e);
	}

	render() {
		return (
			<div className="overlay" onClick={(e) => this.onClick(e)}>
				{this.props.children}
			</div>
		);
	}
}

export default Overlay;