import React from 'react';
import $ from 'jquery';
import Gallery from "react-photo-gallery";

class PhotoGallery extends React.Component {

	constructor(props) {
		super(props);
	}

	render() {
		// Todo: handle resize event to update column count
		const width = $("body").width();
		const roughWidthPerPhoto = 300;		// How width each photo should be, roughly (in pixels)
		const galleryColumns = Math.ceil(width / roughWidthPerPhoto);

		return (
			<div className="photoGallery">
				<Gallery className="" photos={this.props.photos} onClick={(e, d) => { !!this.props.onClick && this.props.onClick(e, d);}} columns={galleryColumns} margin={3} />
			</div>
		);
	}
}

export default PhotoGallery;