import React from 'react';
import PageLayout from "../components/PageLayout.jsx"
import AppStateContext from '../contexts/AppStateContext.jsx';


class SharedPage extends React.Component {

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
				TODO; do this when everything else works great.
			</PageLayout>
		);
	}
}

SharedPage.contextType = AppStateContext;
export default SharedPage;