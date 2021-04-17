import * as React from "react";
import Exif from "../models/Exif";

interface ExifDataProps {
	exif: Exif
}

class ExifData extends React.Component<ExifDataProps> {
	constructor(props: ExifDataProps) {
		super(props);
	}

	createLocationUri(lat: number, lon: number): string {
		return "https://www.openstreetmap.org/#map=18/" + lat + "/" + lon;
	}

	render(): React.ReactNode {
		// Prepare exposure display text
		let exposureText = "";
		if (this.props.exif.aperture)
			exposureText += this.props.exif.aperture + " ";
		if (this.props.exif.exposureTime)
			exposureText += this.props.exif.exposureTime + " ";
		if (this.props.exif.iso)
			exposureText += "ISO-" + this.props.exif.iso;

		// Prepare focal length display text
		let focalLengthText = null;
		if (this.props.exif.focalLength) {
			focalLengthText = this.props.exif.focalLength + "mm";
			if (this.props.exif.focalLength35mmEquiv) {
				focalLengthText += " (" + this.props.exif.focalLength35mmEquiv + "mm in 35mm equivalent)";
			}

		}

		// Prepare date taken display text
		let dateTakenText = null;
		if (this.props.exif.dateTaken) {
			const date = new Date(this.props.exif.dateTaken);
			// This check verifies if parsing from string was succesfull
			if (!isNaN(date.getTime())) {
				dateTakenText = date.toLocaleString();
			}
		}

		// Prepare location display text
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

			{!!exposureText && <div className="property">
				<span className="name">Exposure</span>
				<span className="value">{exposureText}</span>
			</div>}

			{!!focalLengthText && <div className="property">
				<span className="name">Focal length</span>
				<span className="value">{focalLengthText}</span>
			</div>}

			{!!dateTakenText && <div className="property">
				<span className="name">Taken on</span>
				<span className="value">{dateTakenText}</span>
			</div>}

			{!!locationText && <div className="property">
				<span className="name">Location</span>
				<a className="value" target="_blank" href={locationUri as string} rel="noreferrer">{locationText}</a>
			</div>}
		</div>;
	}
}

export default ExifData;