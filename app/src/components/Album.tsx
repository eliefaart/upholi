import * as React from "react";
import AppStateContext from "../contexts/AppStateContext";
import { AlbumNew } from "../models/Album";
import upholiService from "../services/UpholiService";

interface Props {
	onClick: (album: AlbumNew) => void,
	className?: string,
	album: AlbumNew
}

interface State {
	thumbnailSrc: string
}

export default class Album extends React.Component<Props, State> {
	static contextType = AppStateContext;

	constructor(props: Props) {
		super(props);

		if (this.props.album.thumbnailPhotoId) {
			upholiService.getPhotoThumbnailImageSrc(this.props.album.thumbnailPhotoId)
				.then(src => {
					this.setState({
						thumbnailSrc: src
					});
				});
		}

		this.state = {
			thumbnailSrc: ""
		};
	}

	render(): React.ReactNode {
		const album = this.props.album;
		const thumbUrl = this.state.thumbnailSrc ? `url('${this.state.thumbnailSrc}')` : "";

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