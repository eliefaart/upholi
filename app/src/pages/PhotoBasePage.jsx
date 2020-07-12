import React from 'react';
import PhotoService from '../services/PhotoService';
import PhotoDetail from '../components/PhotoDetail.jsx';
import PageLayout from "../components/PageLayout.jsx"
import AppStateContext from '../contexts/AppStateContext.jsx';
import { IconClose, IconDownload } from "../components/Icons.jsx";

class PhotoBasePage extends React.Component {

	constructor(props) {
		super(props);

		this.state = {
			url: this.props.previewUrl,
			downloadUrl: this.props.downloadUrl
		};
	}

	componentDidMount() {
		let fnOnPhotoDataReceived = (photo) => {
			this.setState({ photo });
		};

		PhotoService.getPhotoInfo(this.props.infoUrl)
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
				<PhotoDetail src={this.state.url} exif={!!this.state.photo ? this.state.photo.exif : null} />
			</PageLayout>
		);
	}
}

PhotoBasePage.contextType = AppStateContext;
export default PhotoBasePage;