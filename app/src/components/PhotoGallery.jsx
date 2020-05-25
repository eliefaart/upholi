import React from 'react';
import $ from 'jquery';
import Gallery from "react-photo-gallery";

class PhotoGallery extends React.Component {

	constructor(props) {
		super(props);

		this.state = {
		}
	}

	componentDidMount() {
	}

	componentWillUnmount() {
	}
	
	onPhotoDetailClicked() {
	}

	render() {

		// Todo: handle resize event to update column count
		const width = $("body").width();
		let galleryColumns = 2;
		if (width >= 900)
			galleryColumns = 3;
		if (width >= 1200)
			galleryColumns = 4;
		if (width >= 1500)
			galleryColumns = 5;
		if (width >= 1800)
			galleryColumns = 6;

		return (
			<div className="photoGallery">
				<Gallery className="" photos={this.props.photos} onClick={(e, d) => { !!this.props.onClick && this.props.onClick(e, d);}} columns={galleryColumns} margin={3} />
			</div>
		);
	}
}

export default PhotoGallery;