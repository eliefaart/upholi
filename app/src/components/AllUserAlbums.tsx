import * as React from "react";
import PhotoService from "../services/PhotoService";
import Albums from "../components/Albums";
import AlbumInfo from "../models/AlbumInfo";

interface Props {
	onClick: (album: AlbumInfo) => void
}

interface State {
	albums: AlbumInfo[],
}

export default class AllUserAlbums extends React.Component<Props, State> {

	constructor(props: Props) {
		super(props);

		PhotoService.getAlbums()
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