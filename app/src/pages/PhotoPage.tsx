import * as React from "react";
import { PageBaseComponent, PageBaseComponentProps } from "../components/PageBaseComponent";
import PhotoService from "../services/PhotoService";
import PhotoDetail from "../components/PhotoDetail";
import ContentContainer from "../components/ContentContainer"
import AppStateContext from "../contexts/AppStateContext";
import { IconClose, IconDownload } from "../components/Icons";
import Photo from "../entities/Photo";

interface PhotoPageState {
	photo: Photo | null,
	previewUrl: string,
	downloadUrl: string
}

class PhotoPage extends PageBaseComponent<PhotoPageState> {

	photoId: string;

	constructor(props: PageBaseComponentProps) {
		super(props);

		this.photoId = props.match.params.photoId;
		const photoBaseUrl = PhotoService.baseUrl() + "/photo/" + this.photoId;
		const previewUrl = photoBaseUrl + "/preview";
		const downloadUrl = photoBaseUrl + "/original";

		this.state = {
			photo: null,
			previewUrl,
			downloadUrl
		};
	}

	getHeaderActions() {
		return <React.Fragment>
			{<a className="iconOnly asButton" href={this.state.downloadUrl} download title="Download">
				<IconDownload/>
			</a>}
			{<button className="iconOnly" onClick={() => this.context.history.goBack()} title="Close">
				<IconClose/>
			</button>}
		</React.Fragment>;
	}

	getTitle() {
		return "Photo - " + this.photoId;
	}

	componentDidMount() {
		let fnOnPhotoDataReceived = (photo: Photo) => {
			this.setState({ photo });
		};

		PhotoService.getPhotoInfo(this.photoId)
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