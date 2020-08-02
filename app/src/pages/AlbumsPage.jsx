import React from "react";
import Albums from "../components/Albums.jsx";
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
			{<button className="iconOnly" onClick={(e) => this.onCreateAlbumClick()} title="Create album">
				<IconCreate/>
			</button>}
		</div>);

		return (
			<PageLayout title="Albums" requiresAuthentication={true} headerActions={headerContextMenuActions}>
				<Albums/>

				<ModalCreateAlbum
					isOpen={this.state.newAlbumDialogOpen}
					onRequestClose={() => this.setState({newAlbumDialogOpen: false})}/>
			</PageLayout>
		);
	}
}

export default AlbumsPage;