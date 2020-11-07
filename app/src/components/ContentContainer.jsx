import React from "react";

/**
 * I feel like I don't really need this class..
 * Perhaps I can include this functionality in PageBaseComponent somehow? Or AppBody?
 */
class ContentContainer extends React.Component {

	constructor(props) {
		super(props);
	}

	getClassName() {
		let className = "";

		if (this.props.paddingTop === true){
			className += "padding-top";
		}

		if (className.trim() === ""){
			className = null;
		}

		return className;
	}

	render() {
		const className = this.getClassName();

		return (<main id="content"
			className={className}
			onDrop={this.props.onDrop}
			onDragOver={this.props.onDragOver || ((event) => event.preventDefault())}>
			{this.props.children}
		</main>);
	}
}

export default ContentContainer;