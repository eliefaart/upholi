import React from "react";
import PageLayout from "../components/PageLayout.jsx"
import AppStateContext from "../contexts/AppStateContext.jsx";
import PhotoService from "../services/PhotoService.js"
import CollectionView from "./CollectionView.jsx";

class CollectionPage extends React.Component {

	constructor(props) {
		super(props);

		PhotoService.getCollection(props.match.params.collectionId)
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
			<PageLayout title={this.state.collection.title} requiresAuthentication={false} renderMenu={true}>
				<CollectionView collection={this.state.collection} initialActiveAlbumId={this.props.match.params.albumId}/>
			</PageLayout>
		);
	}
}

CollectionPage.contextType = AppStateContext;
export default CollectionPage;