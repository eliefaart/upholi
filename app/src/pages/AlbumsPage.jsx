import React from "react";
import PageBaseComponent from "../components/PageBaseComponent.jsx";
import AllUserAlbums from "../components/AllUserAlbums.jsx";
import ContentContainer from "../components/ContentContainer.jsx"
import ModalCreateAlbum from "../components/ModalCreateAlbum.jsx"
import AppStateContext from "../contexts/AppStateContext.jsx";

class AlbumsPage extends PageBaseComponent {

	constructor(props) {
		super(props);

		this.state = {
			newAlbumDialogOpen: false
		}
	}

	getHeaderActions() {
		return (<React.Fragment>
			{<button onClick={(e) => this.onCreateAlbumClick()} title="Create album">
				New album
			</button>}
		</React.Fragment>);
	}

	getTitle() {
		return "Albums";
	}

	onCreateAlbumClick() {
		this.setState({
			newAlbumDialogOpen: true
		});
	}

	render() {
		const history = this.context.history;

		return (
			<ContentContainer paddingTop={true}>
				<AllUserAlbums onClick={album => history.push("/album/" + album.id)}/>

				<ModalCreateAlbum
					isOpen={this.state.newAlbumDialogOpen}
					onRequestClose={() => this.setState({newAlbumDialogOpen: false})}/>
			</ContentContainer>
		);
	}
}

AlbumsPage.contextType = AppStateContext;
export default AlbumsPage;