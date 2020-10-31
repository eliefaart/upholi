import React from "react";

/**
 * I feel like I don't really need this class..
 * Perhaps I can include this functionality in PageBaseComponent somehow? Or AppBody?
 */
class ContentContainer extends React.Component {

	constructor(props) {
		super(props);
	}

	render() {
		return (<div id="content"
			onDrop={this.props.onDrop}
			onDragOver={this.props.onDragOver || ((event) => event.preventDefault())}>
			{this.props.children}
		</div>);
	}
}

export default ContentContainer;