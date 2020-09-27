import React from "react";
import PageLayout from "../components/PageLayout.jsx"
import AppStateContext from "../contexts/AppStateContext.jsx";
import PhotoService from "../services/PhotoService";
import Albums from "../components/Albums.jsx";

class SharedCollectionPageNew extends React.Component {

	constructor(props) {
		super(props);

		PhotoService.getCollectionByShareToken(props.match.params.token)
			.then((collection) => this.setState({ collection }))
			.catch(console.error);

		this.state = {};
	}

	render() {
		if (this.state.collection == null)
			return null;

		const history = this.context.history;

		return (
			<PageLayout title={"Collection - " + this.state.title} requiresAuthentication={false} renderMenu={false}>
				<div className="topBar">
					<h1>{this.state.collection.title}</h1>
				</div>
				<div className="collectionContent">
					<Albums 
						albums={this.state.collection.albums}
						onClick={album => history.push(location.pathname + "/album/" + album.id)}/>
				</div>
			</PageLayout>
		);
	}
}

SharedCollectionPageNew.contextType = AppStateContext;
export default SharedCollectionPageNew;