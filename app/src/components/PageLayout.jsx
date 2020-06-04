import React from 'react';
import Header from './Header.jsx';
import { ToastContainer } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';

class PageLayout extends React.Component {

	constructor(props) {
		super(props);
	}

	componentDidMount() {
	}

	componentWillUnmount() {
	}

	render() {
		return (
			<div className="page" 
				onDrop={this.props.onDrop} 
				onDragOver={this.props.onDragOver || ((event) => event.preventDefault())}>
				<Header title={this.props.title} >
					{this.props.headerElementActions}
				</Header>

				<div className="content">
					{this.props.children}
				</div>
				<ToastContainer position="bottom-right"
					autoClose={5000}
					hideProgressBar
					newestOnTop
					closeOnClick
					rtl={false}
					pauseOnFocusLoss
					draggable
					pauseOnHover/>
			</div>
		);
	}
}

export default PageLayout;