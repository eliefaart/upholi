import * as React from "react";
import { PageBaseComponent, PageBaseComponentProps } from "./PageBaseComponent";
import AllUserAlbums from "../AllUserAlbums";
import ContentContainer from "../ContentContainer";
import ModalCreateAlbum from "../modals/ModalCreateAlbum";
import AppStateContext from "../../contexts/AppStateContext";

interface AlbumsPageState {
	newAlbumDialogOpen: boolean
}

class AlbumsPage extends PageBaseComponent<AlbumsPageState> {

	constructor(props: PageBaseComponentProps) {
		super(props);

		this.state = {
			newAlbumDialogOpen: false
		}
	}

	getHeaderActions(): JSX.Element {
		return <React.Fragment>
			{<button onClick={(e) => this.onCreateAlbumClick()} title="Create album">
				New album
			</button>}
		</React.Fragment>;
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