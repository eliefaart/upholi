import React from "react";
import PhotoService from "../services/PhotoService.js"
import Albums from "../components/Albums.jsx";

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
		return <Albums albums={this.state.albums}/>
	}
}

export default AllUserAlbums;