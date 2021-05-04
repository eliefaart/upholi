import * as React from "react";
import AlbumInfo from "../models/AlbumInfo";
import PhotoService from "../services/PhotoService";
import AppStateContext from "../contexts/AppStateContext";

interface Props {
	onClick: (album: AlbumInfo) => void,
	className?: string,
	album: AlbumInfo
}

export default class Album extends React.Component<Props> {
	static contextType = AppStateContext;

	constructor(props: Props) {
		super(props);
	}

	render(): React.ReactNode {
		const album = this.props.album;
		const thumbUrl = album.thumbPhotoId ?"url('" + PhotoService.baseUrl() + "/photo/" + album.thumbPhotoId + "/thumb')" : "";

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