import React from "react";
import PhotoService from "../services/PhotoService";
import AppStateContext from "../contexts/AppStateContext.jsx";
import PhotoBasePage from "./PhotoBasePage.jsx"

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
				requiresAuthentication={false}
				/>
		);
	}
}

PhotoPage.contextType = AppStateContext;
export default PhotoPage;