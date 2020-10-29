import React from "react";
import PageLayout from "../components/PageLayout.jsx"
import AppStateContext from "../contexts/AppStateContext.jsx";
import PhotoService from "../services/PhotoService.js"
import Albums from "../components/Albums.jsx";

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

		const history = this.context.history;

		return (
			<PageLayout title={this.state.collection.title} requiresAuthentication={false} renderMenu={true}>
				<div className="topBar">
					<h1>{this.state.collection.title}</h1>
				</div>
				<div className="collectionContent">
					<Albums
						albums={this.state.collection.albums}
						onClick={album => history.push("/album/" + album.id)}/>
				</div>
			</PageLayout>
		);
	}
}

CollectionPage.contextType = AppStateContext;
export default CollectionPage;