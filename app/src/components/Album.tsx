import * as React from "react";
import PhotoService from "../services/PhotoService";
import AppStateContext from "../contexts/AppStateContext";
import { AlbumNew } from "../models/Album";

interface Props {
	onClick: (album: AlbumNew) => void,
	className?: string,
	album: AlbumNew
}

export default class Album extends React.Component<Props> {
	static contextType = AppStateContext;

	constructor(props: Props) {
		super(props);
	}

	render(): React.ReactNode {
		const album = this.props.album;
		const thumbUrl = album.thumbnailPhotoId ?"url('" + PhotoService.baseUrl() + "/photo/" + album.thumbnailPhotoId + "/thumb')" : "";

		return <div
			onClick={() => this.props.onClick(album)}
			className={"album " + (this.props.className || "")}>
			<div className="album-thumbnail"
				style={{ backgroundImage: thumbUrl }}>
			</div>
			<span title={album.title} className="album-title">{album.title}</span>
		</div>;
	}
}