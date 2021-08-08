// import * as React from "react";
// import { PageBaseComponent, PageBaseComponentProps } from "./PageBaseComponent";
// import PhotoService from "../../services/PhotoService";
// import PhotoDetail from "../PhotoDetail";
// import ContentContainer from "../ContentContainer";
// import AppStateContext from "../../contexts/AppStateContext";
// import { IconClose, IconDownload } from "../Icons";
// import Photo from "../../models/Photo";

// interface PhotoPageState {
// 	photo: Photo | null,
// 	previewUrl: string,
// 	downloadUrl: string
// }

// class PhotoPage extends PageBaseComponent<PhotoPageState> {

// 	photoId: string;

// 	constructor(props: PageBaseComponentProps) {
// 		super(props);

// 		this.photoId = props.match.params.photoId;
// 		const photoBaseUrl = PhotoService.baseUrl() + "/photo/" + this.photoId;
// 		const previewUrl = photoBaseUrl + "/preview";
// 		const downloadUrl = photoBaseUrl + "/original";

// 		this.state = {
// 			photo: null,
// 			previewUrl,
// 			downloadUrl
// 		};
// 	}

// 	getHeaderActions(): JSX.Element | null {
// 		return <React.Fragment>
// 			{<a className="iconOnly asButton" href={this.state.downloadUrl} download title="Download">
// 				<IconDownload/>
// 			</a>}
// 			{<button className="iconOnly" onClick={() => this.context.history.goBack()} title="Close">
// 				<IconClose/>
// 			</button>}
// 		</React.Fragment>;
// 	}

// 	getTitle(): string {
// 		return "Photo - " + this.photoId;
// 	}

// 	componentDidMount(): void {
// 		const fnOnPhotoDataReceived = (photo: Photo) => {
// 			this.setState({ photo });
// 		};

// 		PhotoService.getPhotoInfo(this.photoId)
// 			.then(fnOnPhotoDataReceived)
// 			.catch(console.error);

// 		super.componentDidMount();
// 	}

// 	render(): React.ReactNode {
// 		return (
// 			<ContentContainer>
// 				<PhotoDetail
// 					src={this.state.previewUrl}
// 					isVideo={!!this.state.photo && this.state.photo.contentType.startsWith("video/")}
// 					exif={this.state.photo ? this.state.photo.exif : null} />
// 			</ContentContainer>
// 		);
// 	}
// }

// PhotoPage.contextType = AppStateContext;
// export default PhotoPage;