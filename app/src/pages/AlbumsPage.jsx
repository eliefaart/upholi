import React from "react";
import AllUserAlbums from "../components/AllUserAlbums.jsx";
import PageLayout from "../components/PageLayout.jsx"
import ModalCreateAlbum from "../components/ModalCreateAlbum.jsx"
import AppStateContext from "../contexts/AppStateContext.jsx";

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
		const history = this.context.history;
		const headerContextMenuActions = (<div>
			{<button onClick={(e) => this.onCreateAlbumClick()} title="Create album">
				New album
			</button>}
		</div>);

		return (
			<PageLayout title="Albums" requiresAuthentication={true} headerActions={headerContextMenuActions}>
				<AllUserAlbums onClick={album => history.push("/album/" + album.id)}/>

				<ModalCreateAlbum
					isOpen={this.state.newAlbumDialogOpen}
					onRequestClose={() => this.setState({newAlbumDialogOpen: false})}/>
			</PageLayout>
		);
	}
}

AlbumsPage.contextType = AppStateContext;
export default AlbumsPage;