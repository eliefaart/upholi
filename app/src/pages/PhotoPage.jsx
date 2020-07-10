import React from 'react';
import PhotoService from '../services/PhotoService';
import AppStateContext from '../contexts/AppStateContext.jsx';
import PhotoBasePage from "./PhotoBasePage.jsx"

/// Displays a photo that belongs to the current user
class PhotoPage extends React.Component {

	constructor(props) {
		super(props);
		this.photoId = props.match.params.photoId;
	}

	render() {
		const infoUrl = PhotoService.baseUrl() + "/photo/" + this.photoId;
		const previewUrl = infoUrl + "/preview";
		const downloadUrl = infoUrl + "/original";

		return (
			<PhotoBasePage
				infoUrl={infoUrl}
				previewUrl={previewUrl}
				downloadUrl={downloadUrl}
				requiresAuthentication={true}
				/>
		);
	}
}

PhotoPage.contextType = AppStateContext;
export default PhotoPage;