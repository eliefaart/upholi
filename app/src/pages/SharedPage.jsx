import React from "react";
import PageLayout from "../components/PageLayout.jsx"
import AppStateContext from "../contexts/AppStateContext.jsx";


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
			<PageLayout requiresAuthentication={true}>
				TODO; This should display which albums and which collections are shared for user
			</PageLayout>
		);
	}
}

SharedPage.contextType = AppStateContext;
export default SharedPage;