import * as React from "react";
import { FC } from "react";
import { AlbumNew } from "../../models/Album";
import upholiService from "../../services/UpholiService";

interface Props {
	onClick: (album: AlbumNew) => void,
	className?: string,
	album: AlbumNew
}

const Album: FC<Props> = (props) => {
	const [thumbnailSrc, setThumbnailSrc] = React.useState("");

	upholiService.getPhotoThumbnailImageSrc(props.album.thumbnailPhotoId)
		.then(setThumbnailSrc);

	const thumbUrl = `url('${thumbnailSrc}')`;

	return <div
		onClick={() => props.onClick(props.album)}
		className={"album " + (props.className || "")}>
		<div className="album-thumbnail"
			style={{ backgroundImage: thumbUrl }}>
		</div>
		<span title={props.album.title} className="album-title">{props.album.title}</span>
	</div>;
};

export default Album;