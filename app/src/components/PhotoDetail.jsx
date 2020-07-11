import React from 'react';
import ExifData from '../components/ExifData.jsx';

class PhotoDetail extends React.Component {
	constructor(props) {
		super(props);
	}

	componentDidMount() {
		// TODO: Center the photo displayed.
		// EITHER center photo within iframe, but struggling with that. I can center it, but then I can no longer zoom.
		// OR center iframe within parent, then I need to implement iframe sending messages to parent about what size it wants to be.

		window.frames[0].document.body.onload = () => {
			// Apply some styling to the elements inside the iframe, to get rid of unwanted margins etc.

			const iframeBody = window.frames[0].document.body;
			iframeBody.style.margin = "0px";

			// const imgElement = iframeBody.getElementsByTagName("img")[0];
		};
	}

	render() {
		return <div className="photoDetail">
			{this.props.exif && <ExifData exif={this.props.exif}/>}
			<iframe src={this.props.src}/>
		</div>;
	}
}

export default PhotoDetail;