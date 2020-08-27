import React from "react";
import AllUserAlbums from "../components/AllUserAlbums.jsx";
import PageLayout from "../components/PageLayout.jsx"
import ModalCreateAlbum from "../components/ModalCreateAlbum.jsx"
import { IconCreate } from "../components/Icons.jsx";

class AlbumsPage extends React.Component {

	constructor(props) {
		super(props);

		this.state = {
			newAlbumDialogOpen: false
		}
	}

	onCreateAlbumClick() {
		this.setState({
			newAlbumDialogOpen: true
		});
	}

	render() {
		const headerContextMenuActions = (<div>
			{<button onClick={(e) => this.onCreateAlbumClick()} title="Create album">
				New album
			</button>}
		</div>);

		return (
			<PageLayout title="Albums" requiresAuthentication={true} headerActions={headerContextMenuActions}>
				<AllUserAlbums/>

				<ModalCreateAlbum
					isOpen={this.state.newAlbumDialogOpen}
					onRequestClose={() => this.setState({newAlbumDialogOpen: false})}/>
			</PageLayout>
		);
	}
}

export default AlbumsPage;