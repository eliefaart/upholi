import React from 'react';
import PhotoService from '../services/PhotoService';
import PhotoDetail from '../components/PhotoDetail.jsx';
import PageLayout from "../components/PageLayout.jsx"
import AppStateContext from '../contexts/AppStateContext.jsx';
import { IconClose, IconDownload } from "../components/Icons.jsx";

class PhotoPage extends React.Component {

	constructor(props) {
		super(props);

		this.photoId = props.match.params.photoId;

		this.state = {
			url: PhotoService.baseUrl() + "/photo/" + props.match.params.photoId + "/preview",
			downloadUrl: PhotoService.baseUrl() + "/photo/" + props.match.params.photoId + "/original"
		};
	}

	componentDidMount() {
		let fnOnPhotoDataReceived = (photo) => {
			this.setState({ photo });
		};

		PhotoService.getPhoto(this.photoId)
			.then(fnOnPhotoDataReceived)
			.catch(console.error);
	}

	componentWillUnmount() {
	}

	render() {
		const headerActions = (<div>
			{<a className="iconOnly" href={this.state.downloadUrl} download>
				<IconDownload/>
			</a>}
			{<button className="iconOnly" onClick={() => this.context.history.goBack()}>
				<IconClose/>
			</button>}
			
		</div>);

		return (
			<PageLayout renderMenu={false} headerElementActions={headerActions}>
				<PhotoDetail src={this.state.url} exif={!!this.state.photo ? this.state.photo.exif : null} />
			</PageLayout>
		);
	}
}

PhotoPage.contextType = AppStateContext;
export default PhotoPage;