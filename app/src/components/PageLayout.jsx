import React from 'react';
import { default as ReactModal } from 'react-modal';
import Header from './Header.jsx';

class PageLayout extends React.Component {

	constructor(props) {
		super(props);
	}

	componentDidMount() {
		ReactModal.setAppElement("#app");
	}

	render() {
		return (
			<div className="page" 
				onDrop={this.props.onDrop} 
				onDragOver={this.props.onDragOver || ((event) => event.preventDefault())}>
				<Header title={this.props.title} 
					renderMenu={this.props.renderMenu} 
					actionsElement={this.props.headerActions}
					contextMenuElement={this.props.headerContextMenuActions}
					>
				</Header>

				<div className="content">
					{this.props.children}
				</div>
			</div>
		);
	}
}

export default PageLayout;