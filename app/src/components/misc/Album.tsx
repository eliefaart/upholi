import * as React from "react";
import { FC } from "react";
import usePhotoThumbnailSource from "../../hooks/usePhotoThumbnailSource";
import { AlbumNew } from "../../models/Album";

interface Props {
	onClick: (album: AlbumNew) => void,
	album: AlbumNew
}

const Album: FC<Props> = (props) => {
	const thumbnailSrc = usePhotoThumbnailSource(props.album.thumbnailPhotoId);
	const thumbUrl = `url('${thumbnailSrc}')`;

	return <div
		onClick={() => props.onClick(props.album)}
		className="album">
		<div className="album-thumbnail"
			style={{ backgroundImage: thumbUrl }}>
		</div>
		<span title={props.album.title} className="album-title">{props.album.title}</span>
	</div>;
};

export default Album;