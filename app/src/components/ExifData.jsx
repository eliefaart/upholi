import React from 'react';

class ExifData extends React.Component {
	constructor(props) {
		super(props);
	}

	createLocationUri(lat, lon) {
		return "https://www.openstreetmap.org/#map=18/" + lat + "/" + lon;
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

		let locationText = null, locationUri = null;
		if (!!this.props.exif.gpsLatitude && !!this.props.exif.gpsLongitude) {
			locationText = this.props.exif.gpsLatitude + ", " + this.props.exif.gpsLongitude;
			locationUri = this.createLocationUri(this.props.exif.gpsLatitude, this.props.exif.gpsLongitude);
		}

		return !!this.props.exif && <div className="exif">
			{!!this.props.exif.manufactorer && <div className="property">
				<span className="name">Camera</span>
				<span className="value">{this.props.exif.manufactorer} {this.props.exif.model}</span>
			</div>}

			{!!this.props.exif.aperture && <div className="property">
				<span className="name">Exposure</span>
				<span className="value">{this.props.exif.aperture} {this.props.exif.exposureTime} ISO-{this.props.exif.iso}</span>
			</div>}

			{!!dateTakenPretty && <div className="property">
				<span className="name">Taken on</span>
				<span className="value">{dateTakenPretty}</span>
			</div>}

			{!!locationText && <div className="property">
				<span className="name">Location</span>
				<a className="value" target="_blank" href={locationUri}>{locationText}</a>
			</div>}
		</div>;
	}
}

export default ExifData;