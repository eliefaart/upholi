import React from "react";
import PhotoService from "../services/PhotoService";
import AppStateContext from "../contexts/AppStateContext.jsx";
import PhotoBasePage from "./PhotoBasePage.jsx"

/// Displays a photo that is part of a shared collection
class SharedCollectionPhotoPage extends React.Component {

	constructor(props) {
		super(props);
		this.photoId = props.match.params.photoId;
		this.collectionId = props.match.params.collectionId;
	}

	render() {
		const infoUrl = PhotoService.baseUrl() + "/pub/collection/" + this.collectionId + "/photo/" + this.photoId;
		const previewUrl = infoUrl + "/preview";
		const downloadUrl = infoUrl + "/original";

		return (
			<PhotoBasePage
				infoUrl={infoUrl}
				previewUrl={previewUrl}
				downloadUrl={downloadUrl}
				requiresAuthentication={false}
				/>
		);
	}
}

SharedCollectionPhotoPage.contextType = AppStateContext;
export default SharedCollectionPhotoPage;