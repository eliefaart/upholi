import * as React from "react";
import { PageBaseComponent, PageBaseComponentProps } from "../components/PageBaseComponent";
import ContentContainer from "../components/ContentContainer"
import AppStateContext from "../contexts/AppStateContext";
import CollectionView from "../components/CollectionView";
import Collection from "../entities/Collection";

interface CollectionPageBaseState {
	collection: Collection | null
}

class CollectionPageBase extends PageBaseComponent<CollectionPageBaseState> {

	constructor(props: PageBaseComponentProps, collectionPromise: Promise<Collection>) {
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