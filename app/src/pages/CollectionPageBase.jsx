import React from "react";
import PageBaseComponent from "../components/PageBaseComponent.jsx";
import ContentContainer from "../components/ContentContainer.jsx"
import AppStateContext from "../contexts/AppStateContext.ts";
import CollectionView from "./CollectionView.jsx";

class CollectionPageBase extends PageBaseComponent {

	constructor(props, collectionPromise) {
		super(props);

		collectionPromise
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

CollectionPageBase.contextType = AppStateContext;
export default CollectionPageBase;