import * as React from "react";
import PhotoService from "../services/PhotoService"
import Albums from "../components/Albums";
import Album from "../entities/Album";

interface AllUserAlbumsProps {
	onClick: (event: React.MouseEvent<HTMLElement, MouseEvent>) => void
}

interface AllUserAlbumsState {
	albums: Album[],
}

class AllUserAlbums extends React.Component<AllUserAlbumsProps, AllUserAlbumsState> {

	constructor(props: AllUserAlbumsProps) {
		super(props);

		let _this = this;
		PhotoService.getAlbums()
			.then((albums) => _this.setState({ albums: albums }))
			.catch(console.error);

		this.state = {
			albums: []
		};
	}

	render() {
		return <Albums albums={this.state.albums} onClick={this.props.onClick}/>
	}
}

export default AllUserAlbums;