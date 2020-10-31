import React from "react";
import PageBaseComponent from "../components/PageBaseComponent.jsx";
import ContentContainer from "../components/ContentContainer.jsx"
import AppStateContext from "../contexts/AppStateContext.jsx";
import PhotoService from "../services/PhotoService.js"
import CollectionView from "./CollectionView.jsx";

class CollectionPage extends PageBaseComponent {

	constructor(props) {
		super(props);

		PhotoService.getCollection(props.match.params.collectionId)
			.then((collection) => this.setState({ collection }))
			.catch(console.error);

		this.state = {
			collection: null
		};
	}

	getTitle() {
		return "Collection - " + this.state.collection.title;
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

CollectionPage.contextType = AppStateContext;
export default CollectionPage;