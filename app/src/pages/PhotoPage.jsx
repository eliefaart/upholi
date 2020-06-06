import React from 'react';
import PhotoService from '../services/PhotoService';
import PhotoDetail from '../components/PhotoDetail.jsx';
import PageLayout from "../components/PageLayout.jsx"
import AppStateContext from '../contexts/AppStateContext.jsx';
import { IconClose, IconDownload } from "../components/Icons.jsx";

class PhotoPage extends React.Component {

	constructor(props) {
		super(props);

		this.state = {
			url: PhotoService.baseUrl() + "/photo/" + props.match.params.photoId + "/preview",
			downloadUrl: PhotoService.baseUrl() + "/photo/" + props.match.params.photoId + "/original"
		};
	}

	componentDidMount() {
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
				<div>
					EXIF BE HERE
				</div>
				<PhotoDetail src={this.state.url} />
			</PageLayout>
		);
	}
}

PhotoPage.contextType = AppStateContext;
export default PhotoPage;