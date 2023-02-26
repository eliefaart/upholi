import * as React from "react";
import { FC } from "react";
import { IconCheck } from "../misc/Icons";
import { default as GalleryPhotoModel } from "../../models/GalleryPhoto";
import usePhotoThumbnailSource from "../../hooks/usePhotoThumbnailSource";

interface Props {
  photo: GalleryPhotoModel;
  margin?: string;

  selected: boolean;
  anySiblingPhotoSelected: boolean;

  onClick: () => void;
  onToggleSelect: () => void;
}

const GalleryPhoto: FC<Props> = (props: Props) => {
  const photoId = props.photo.id ?? "";
  const src = usePhotoThumbnailSource(props.photo.mayLoad ? photoId : "");

  if (!photoId) {
    return <></>;
  } else {
    const cssClass =
      "photo" + (props.selected ? " selected" : "") + (props.anySiblingPhotoSelected ? " any-other-selected" : "");
    const imgStyle: React.CSSProperties = {
      backgroundImage: 'url("' + src + '")',
      margin: props.margin,
      width: props.photo.width,
      height: props.photo.height,
    };

    return (
      <div key={photoId} className={cssClass}>
        {/* Render a div instead of an img element. This is solely to prevent the default (longpress) context menu to appear in mobile browsers */}
        <div
          id={photoId}
          className="photoImg"
          style={imgStyle}
          onClick={props.onClick}
          onContextMenu={(event) => {
            event.preventDefault();
            props.onToggleSelect();
          }}
        >
          {props.selected && (
            <div className="selected-overlay">
              <IconCheck />
            </div>
          )}
        </div>
      </div>
    );
  }
};

export default GalleryPhoto;
