import * as React from "react";
import { AlbumNew } from "../models/Album";
import appStateContext from "../contexts/AppStateContext";
import Album from "./Album";

interface Props {
	onClick: (album: AlbumNew) => void,
	albums: AlbumNew[]
}

class Albums extends React.Component<Props> {
	static contextType = appStateContext;

	constructor(props: Props) {
		super(props);
	}

	render(): React.ReactNode {
		const albums = this.props.albums.map((album) => (
			<Album key={album.id} album={album} onClick={this.props.onClick} />
		));

		return <div className="albums">
			{albums}
		</div>;
	}
}

Albums.contextType = appStateContext;
export default Albums;