import * as React from "react";
import { FC } from "react";
import Exif from "../../models/Exif";
import { createMapsLocationUri } from "../../utils/location";

interface Props {
  exif: Exif;
}

const ExifData: FC<Props> = (props) => {
  // Prepare exposure display text
  let exposureText = "";
  if (props.exif.aperture) exposureText += props.exif.aperture + " ";
  if (props.exif.exposureTime) exposureText += props.exif.exposureTime + " ";
  if (props.exif.iso) exposureText += "ISO-" + props.exif.iso;

  // Prepare focal length display text
  let focalLengthText = null;
  if (props.exif.focalLength) {
    focalLengthText = props.exif.focalLength + "mm";
    if (props.exif.focalLength35mmEquiv) {
      focalLengthText += " (" + props.exif.focalLength35mmEquiv + "mm in 35mm equivalent)";
    }
  }

  // Prepare date taken display text
  let dateTakenText = null;
  if (props.exif.dateTaken) {
    const date = new Date(props.exif.dateTaken);
    // This check verifies if parsing from string was succesfull
    if (!isNaN(date.getTime())) {
      dateTakenText = date.toLocaleString();
    }
  }

  // Prepare location display text
  let locationText = null,
    locationUri = null;
  if (!!props.exif.gpsLatitude && !!props.exif.gpsLongitude) {
    locationText = props.exif.gpsLatitude + ", " + props.exif.gpsLongitude;
    locationUri = createMapsLocationUri(props.exif.gpsLatitude, props.exif.gpsLongitude);
  }

  return (
    !!props.exif && (
      <div className="exif">
        {!!props.exif.manufactorer && (
          <div className="property">
            <span className="name">Camera</span>
            <span className="value">
              {props.exif.manufactorer} {props.exif.model}
            </span>
          </div>
        )}

        {!!exposureText && (
          <div className="property">
            <span className="name">Exposure</span>
            <span className="value">{exposureText}</span>
          </div>
        )}

        {!!focalLengthText && (
          <div className="property">
            <span className="name">Focal length</span>
            <span className="value">{focalLengthText}</span>
          </div>
        )}

        {!!dateTakenText && (
          <div className="property">
            <span className="name">Taken on</span>
            <span className="value">{dateTakenText}</span>
          </div>
        )}

        {!!locationText && (
          <div className="property">
            <span className="name">Location</span>
            <a className="value" target="_blank" href={locationUri as string} rel="noreferrer">
              {locationText}
            </a>
          </div>
        )}
      </div>
    )
  );
};

export default ExifData;
