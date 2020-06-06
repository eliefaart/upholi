import React from 'react';

import PhotoService from "../services/PhotoService.js"
import AppStateContext from '../contexts/AppStateContext.jsx';

class Albums extends React.Component {

	constructor(props) {
		super(props);

		let _this = this;
		PhotoService.getAlbums()
			.then((albums) => _this.setState({ albums: albums }))
			.catch((error) => console.log(error));

		this.state = {
			albums: []
		};
	}

	componentDidMount() {
	}

	componentWillUnmount() {
	}

	render() {
		let history = this.context.history;
		let fnOnClick = this.props.onClick
			|| ((album) => { history.push("/album/" + album.id) });

		const AlbumLink = function (props) {
			let album = props.album;
			let thumbUrl = "url('" + PhotoService.baseUrl() + "/photo/" + album.thumbPhotoId + "/thumb')";

			return <div onClick={() => fnOnClick(album)} className={"album " + (props.className || "")} style={{ backgroundImage: !!album.thumbPhotoId && thumbUrl }}>
				<span>{album.title}</span>
			</div>;
		}

		const albums = this.state.albums.map((album) => {
			return (
				<AlbumLink key={album.id} album={album} />
			);
		});

		return (
			<div className="photoAlbums">
				{albums}
			</div>
		);
	}
}

Albums.contextType = AppStateContext;
export default Albums;