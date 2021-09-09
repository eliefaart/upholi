import * as React from "react";
import { AlbumNew } from "../models/Album";
import appStateContext from "../contexts/AppStateContext";
import Album from "./Album";

interface AlbumProps {
	onClick: (album: AlbumNew) => void,
	activeAlbumId?: string,
	albumUrl?: (albumUrl: string) => string,
	albums: AlbumNew[]
}

class Albums extends React.Component<AlbumProps> {
	static contextType = appStateContext;

	constructor(props: AlbumProps) {
		super(props);
	}

	render(): React.ReactNode {
		const anyAlbumActive = this.props.albums.some(album => album.id === this.props.activeAlbumId);

		const history = this.context.history;
		const fnOnClick = this.props.onClick || ((album: AlbumNew) => {
			if (this.props.albumUrl) {
				history.push(this.props.albumUrl(album.id));
			}
		});

		const albums = this.props.albums.map((album) => (
			<Album key={album.id} album={album} onClick={fnOnClick} />
		));

		return <div className={"albums " + (anyAlbumActive ? "anyActive" : "")}>
			{albums}
		</div>;
	}
}

Albums.contextType = appStateContext;
export default Albums;