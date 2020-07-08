import React from 'react';
import PhotoService from '../services/PhotoService';
import PhotoDetail from '../components/PhotoDetail.jsx';
import PageLayout from "../components/PageLayout.jsx"
import AppStateContext from '../contexts/AppStateContext.jsx';
import { IconClose, IconDownload } from "../components/Icons.jsx";

class SharedCollectionPhotoPage extends React.Component {

	constructor(props) {
		super(props);

		this.photoId = props.match.params.photoId;
		this.collectionId = props.match.params.collectionId;

		this.state = {
			url: PhotoService.baseUrl() + "/pub/collection/" + props.match.params.collectionId + "/photo/" + props.match.params.photoId + "/preview",
			downloadUrl: PhotoService.baseUrl() + "/pub/collection/" + props.match.params.collectionId + "/photo/" + props.match.params.photoId + "/original"
		};
	}

	componentDidMount() {
		let fnOnPhotoDataReceived = (photo) => {
			this.setState({ photo });
		};

		PhotoService.getSharedCollectionPhoto(this.collectionId, this.photoId)
			.then(fnOnPhotoDataReceived)
			.catch(console.error);
	}

	componentWillUnmount() {
	}

	render() {
		const headerActions = (<div>
			{<a className="iconOnly asButton" href={this.state.downloadUrl} download>
				<IconDownload/>
			</a>}
			{<button className="iconOnly" onClick={() => this.context.history.goBack()}>
				<IconClose/>
			</button>}
			
		</div>);

		return (
			<PageLayout renderMenu={false} headerActions={headerActions}>
				<PhotoDetail src={this.state.url} exif={!!this.state.photo ? this.state.photo.exif : null} />
			</PageLayout>
		);
	}
}

SharedCollectionPhotoPage.contextType = AppStateContext;
export default SharedCollectionPhotoPage;