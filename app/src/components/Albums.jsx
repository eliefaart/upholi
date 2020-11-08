import React from "react";
import PhotoService from "../services/PhotoService.ts"
import AppStateContext from "../contexts/AppStateContext.jsx";

class Albums extends React.Component {

	constructor(props) {
		super(props);
	}

	render() {
		const activeAlbumId = this.props.activeAlbumId;
		const anyAlbumActive = this.props.albums.some(album => album.id === this.props.activeAlbumId);

		const fnOnClick = this.props.onClick
			|| ((album) => { history.push(this.props.albumUrl(album.id)) });

		const AlbumElement = function (props) {
			const album = props.album;
			const thumbUrl = "url('" + PhotoService.baseUrl() + "/photo/" + album.thumbPhotoId + "/thumb')";
			const isActive = album.id === activeAlbumId;

			return <div
				onClick={() => fnOnClick(album)}
				className={"album " + (props.className || "") + (isActive ? " active" : "")}
				style={{ backgroundImage: !!album.thumbPhotoId && thumbUrl }}
				>
					<span>{album.title}</span>
			</div>;
		}

		const albums = this.props.albums.map((album) => {
			return (
				<AlbumElement key={album.id} album={album} />
			);
		});

		return (
			<div className={"albums " + (anyAlbumActive ? "anyActive" : "")}>
				{albums}
			</div>
		);
	}
}

Albums.contextType = AppStateContext;
export default Albums;