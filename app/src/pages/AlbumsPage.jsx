import React from 'react';
import Albums from '../components/Albums.jsx';
import PageLayout from "../components/PageLayout.jsx"

class AlbumsPage extends React.Component {

	constructor(props) {
		super(props);
	}

	componentDidMount() {
	}

	componentWillUnmount() {
	}

	render() {
		return (
			<PageLayout>
				<Albums/>
			</PageLayout>
		);
	}
}

export default AlbumsPage;