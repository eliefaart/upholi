import * as React from "react";
import { FC } from "react";
import usePhotoThumbnailSource from "../../hooks/usePhotoThumbnailSource";
import { AlbumPlain } from "../../models/Album";

interface Props {
	onClick: (album: AlbumPlain) => void,
	album: AlbumPlain
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