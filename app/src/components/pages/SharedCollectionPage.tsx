import * as React from "react";
import { PageBaseComponent, PageBaseComponentProps } from "./PageBaseComponent";
import PhotoService from "../../services/PhotoService";
import ContentContainer from "../ContentContainer"
import AppStateContext from "../../contexts/AppStateContext";
import CollectionView from "../CollectionView";
import Collection from "../../models/Collection";

interface CollectionPageBaseState {
	collection: Collection | null
}

class SharedCollectionPage extends PageBaseComponent<CollectionPageBaseState> {

	readonly collectionToken: string;

	constructor(props: PageBaseComponentProps) {
		super(props);

		this.collectionToken = props.match.params.token;

		PhotoService.getCollectionByShareToken(this.collectionToken)
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

	authenticate(): void {
		PhotoService.authenticateToCollectionByShareToken(this.collectionToken, "ac");
	}

	render() {
		return (
			<ContentContainer>
				<React.Fragment>
					<button onClick={() => this.authenticate()}>authenticate</button>
					{this.state.collection != null && <CollectionView collection={this.state.collection}/>}
				</React.Fragment>
			</ContentContainer>
		);
	}
}

SharedCollectionPage.contextType = AppStateContext;
export default SharedCollectionPage;