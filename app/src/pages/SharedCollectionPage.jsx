import React from "react";
import PageLayout from "../components/PageLayout.jsx"
import AppStateContext from "../contexts/AppStateContext.jsx";
import PhotoService from "../services/PhotoService";
import CollectionView from "./CollectionView.jsx";

class SharedCollectionPage extends React.Component {

	constructor(props) {
		super(props);

		PhotoService.getCollectionByShareToken(props.match.params.token)
			.then((collection) => this.setState({ collection }))
			.catch(console.error);

		this.state = {
			collection: null
		};
	}

	render() {
		if (this.state.collection == null)
			return null;

		return (
			<PageLayout title={"Collection - " + this.state.collection.title} requiresAuthentication={false} renderMenu={false}>
				<CollectionView collection={this.state.collection}/>
			</PageLayout>
		);
	}
}

SharedCollectionPage.contextType = AppStateContext;
export default SharedCollectionPage;