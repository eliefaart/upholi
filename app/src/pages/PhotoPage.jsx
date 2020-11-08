import React from "react";
import PageBaseComponent from "../components/PageBaseComponent.jsx";
import PhotoService from "../services/PhotoService.ts";
import PhotoDetail from "../components/PhotoDetail.jsx";
import ContentContainer from "../components/ContentContainer.jsx"
import AppStateContext from "../contexts/AppStateContext.ts";
import { IconClose, IconDownload } from "../components/Icons.tsx";

class PhotoPage extends PageBaseComponent {
	constructor(props) {
		super(props);

		const photoId = props.match.params.photoId;
		const photoBaseUrl = PhotoService.baseUrl() + "/photo/" + photoId;
		const previewUrl = photoBaseUrl + "/preview";
		const downloadUrl = photoBaseUrl + "/original";

		this.state = {
			photoId,
			previewUrl,
			downloadUrl
		};
	}

	getHeaderActions() {
		return (<React.Fragment>
			{<a className="iconOnly asButton" href={this.state.downloadUrl} download title="Download">
				<IconDownload/>
			</a>}
			{<button className="iconOnly" onClick={() => this.context.history.goBack()} title="Close">
				<IconClose/>
			</button>}
		</React.Fragment>);
	}

	getTitle() {
		return "Photo - " + this.state.photoId;
	}

	componentDidMount() {
		let fnOnPhotoDataReceived = (photo) => {
			this.setState({ photo });
		};

		PhotoService.getPhotoInfo(this.state.photoId)
			.then(fnOnPhotoDataReceived)
			.catch(console.error);

		super.componentDidMount();
	}

	render() {
		return (
			<ContentContainer>
				<PhotoDetail src={this.state.previewUrl} exif={!!this.state.photo ? this.state.photo.exif : null} />
			</ContentContainer>
		);
	}
}

PhotoPage.contextType = AppStateContext;
export default PhotoPage;