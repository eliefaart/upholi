import * as React from "react";
import AlbumInfo from "../entities/AlbumInfo";
import PhotoService from "../services/PhotoService"
import AppStateContext from "../contexts/AppStateContext";

interface AlbumProps {
	onClick: (album: AlbumInfo) => void,
	activeAlbumId?: string,
	albumUrl?: (albumUrl: string) => string,
	albums: AlbumInfo[]
}

interface AlbumElementProps {
	className?: string,
	album: AlbumInfo
}

class Albums extends React.Component<AlbumProps> {
	static contextType = AppStateContext;

	constructor(props: AlbumProps) {
		super(props);
	}

	render() {
		const activeAlbumId = this.props.activeAlbumId;
		const anyAlbumActive = this.props.albums.some(album => album.id === this.props.activeAlbumId);

		const history = this.context.history;
		const fnOnClick = this.props.onClick || ((album: AlbumInfo) => {
			if (this.props.albumUrl) {
				history.push(this.props.albumUrl(album.id));
			}
		});

		// Inline child component. TODO: Just make this a proper component, but only needs to be known within this module.
		const AlbumElement = function (props: AlbumElementProps) {
			const album = props.album;
			if (album) {
				const thumbUrl = "url('" + PhotoService.baseUrl() + "/photo/" + album.thumbPhotoId + "/thumb')";
				const isActive = album.id === activeAlbumId;

				return <div
					onClick={() => fnOnClick(album)}
					className={"album " + (props.className || "") + (isActive ? " active" : "")}
					style={{ backgroundImage: !!album.thumbPhotoId && thumbUrl } as any}>
					<span>{album.title}</span>
				</div>;
			}
			else {
				return null;
			}
		}

		const albums = this.props.albums.map((album) => {
			return (
				<AlbumElement key={album.id} album={album} />
			);
		});

		return <div className={"albums " + (anyAlbumActive ? "anyActive" : "")}>
			{albums}
		</div>;
	}
}

Albums.contextType = AppStateContext;
export default Albums;