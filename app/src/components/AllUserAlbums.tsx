import * as React from "react";
import PhotoService from "../services/PhotoService";
import Albums from "../components/Albums";
import AlbumInfo from "../models/AlbumInfo";

interface AllUserAlbumsProps {
	onClick: (album: AlbumInfo) => void
}

interface AllUserAlbumsState {
	albums: AlbumInfo[],
}

class AllUserAlbums extends React.Component<AllUserAlbumsProps, AllUserAlbumsState> {

	constructor(props: AllUserAlbumsProps) {
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

export default AllUserAlbums;