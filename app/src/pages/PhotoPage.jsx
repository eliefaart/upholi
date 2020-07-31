import React from "react";
import PhotoService from "../services/PhotoService";
import PhotoDetail from "../components/PhotoDetail.jsx";
import PageLayout from "../components/PageLayout.jsx"
import AppStateContext from "../contexts/AppStateContext.jsx";
import { IconClose, IconDownload } from "../components/Icons.jsx";

class PhotoPage extends React.Component {
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

	componentDidMount() {
		let fnOnPhotoDataReceived = (photo) => {
			this.setState({ photo });
		};

		PhotoService.getPhotoInfo(this.state.photoId)
			.then(fnOnPhotoDataReceived)
			.catch(console.error);
	}

	render() {
		const headerActions = (<div>
			{<a className="iconOnly asButton" href={this.state.downloadUrl} download title="Download">
				<IconDownload/>
			</a>}
			{<button className="iconOnly" onClick={() => this.context.history.goBack()} title="Close">
				<IconClose/>
			</button>}
		</div>);

		return (
			<PageLayout requiresAuthentication={this.props.requiresAuthentication} renderMenu={false} headerActions={headerActions}>
				<PhotoDetail src={this.state.previewUrl} exif={!!this.state.photo ? this.state.photo.exif : null} />
			</PageLayout>
		);
	}
}

PhotoPage.contextType = AppStateContext;
export default PhotoPage;