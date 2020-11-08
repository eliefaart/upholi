import React from "react";
import PhotoService from "../services/PhotoService.ts"
import Albums from "../components/Albums.tsx";

class AllUserAlbums extends React.Component {

	constructor(props) {
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