import React from 'react';
import PhotoService from '../services/PhotoService';
import PhotoDetail from '../components/PhotoDetail.jsx';

class PhotoPage extends React.Component {

	constructor(props) {
		super(props);

		this.state = {
			url: PhotoService.baseUrl() + "/photo/" + props.match.params.photoId + "/preview"
		};
	}

	componentDidMount() {
	}

	componentWillUnmount() {
	}

	render() {
		return (
			<PhotoDetail src={this.state.url} />
		);
	}
}

export default PhotoPage;