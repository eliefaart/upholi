import React from "react";
import PageBaseComponent from "../components/PageBaseComponent.jsx";
import ContentContainer from "../components/ContentContainer.jsx"
import AppStateContext from "../contexts/AppStateContext.jsx";
import PhotoService from "../services/PhotoService";
import CollectionView from "./CollectionView.jsx";

class SharedCollectionPage extends PageBaseComponent {

	constructor(props) {
		super(props);

		PhotoService.getCollectionByShareToken(props.match.params.token)
			.then((collection) => this.setState({ collection }))
			.catch(console.error);

		this.state = {
			collection: null
		};
	}

	getTitle() {
		return this.state.collection
			? "Collection - " + this.state.collection.title
			: super.getTitle();
	}

	render() {
		if (this.state.collection == null)
			return null;

		return (
			<ContentContainer>
				<CollectionView collection={this.state.collection}/>
			</ContentContainer>
		);
	}
}

SharedCollectionPage.contextType = AppStateContext;
export default SharedCollectionPage;