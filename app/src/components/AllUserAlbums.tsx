import * as React from "react";
import Albums from "../components/Albums";
import upholiService from "../services/UpholiService";
import { AlbumNew } from "../models/Album";

interface Props {
	onClick: (album: AlbumNew) => void
}

interface State {
	albums: AlbumNew[],
}

export default class AllUserAlbums extends React.Component<Props, State> {

	constructor(props: Props) {
		super(props);

		upholiService.getAlbums()
			.then((albums) => {
				this.setState({
					albums: albums
				});
			})
			.catch(console.error);

		this.state = {
			albums: []
		};
	}

	render(): React.ReactNode {
		return <Albums albums={this.state.albums} onClick={this.props.onClick}/>;
	}
}