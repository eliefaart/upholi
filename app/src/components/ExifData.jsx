import React from 'react';

class ExifData extends React.Component {
	constructor(props) {
		super(props);
	}

	render() {
		let dateTakenPretty = null;
		if (!!this.props.exif.dateTaken) {
			let date = new Date(this.props.exif.dateTaken);
			// This check verifies if parsing from string was succesfull
			if (date.getTime() !== NaN) {
				dateTakenPretty = date.toLocaleString();
			}
		}

		return !!this.props.exif && <div className="exif">
			<div className="property">
				<span className="name">Camera</span>
				<span className="value">{this.props.exif.manufactorer} {this.props.exif.model}</span>
			</div>

			<div className="property">
				<span className="name">Exposure</span>
				<span className="value">{this.props.exif.aperture} {this.props.exif.exposureTime} {this.props.exif.iso}ISO</span>
			</div>

			{!!dateTakenPretty && <div className="property">
				<span className="name">Taken on</span>
				<span className="value">{dateTakenPretty}</span>
			</div>}
		</div>;
	}
}

export default ExifData;