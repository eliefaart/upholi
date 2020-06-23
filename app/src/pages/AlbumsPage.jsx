import React from 'react';
import Albums from '../components/Albums.jsx';
import PageLayout from "../components/PageLayout.jsx"
import ModalCreateAlbum from "../components/ModalCreateAlbum.jsx"

class AlbumsPage extends React.Component {

	constructor(props) {
		super(props);

		this.state = {
			newAlbumDialogOpen: false
		}
	}

	componentDidMount() {
	}

	componentWillUnmount() {
	}

	onCreateAlbumClick() {
		this.setState({
			newAlbumDialogOpen: true
		});
	}

	render() {
		const headerActions = (<div>
			{<button onClick={(e) => this.onCreateAlbumClick()}>Create album</button>}
		</div>);

		return (
			<PageLayout headerContextMenuActions={headerActions}>
				<Albums/>

				<ModalCreateAlbum
					isOpen={this.state.newAlbumDialogOpen}
					onRequestClose={() => this.setState({newAlbumDialogOpen: false})}/>
				{/* {this.state.newAlbumDialogOpen && 
					<Modal
						title="Create album"
						isOpen={this.state.newAlbumDialogOpen}
						onRequestClose={() => this.setState({newAlbumDialogOpen: false})}
						onOkButtonClick={() => this.submitCreateAlbum()}
						okButtonText="Create"
						>
							<form id="form-create-album">
								<input name="title" placeholder="Title"/>
							</form>
					</Modal>} */}
			</PageLayout>
		);
	}
}

export default AlbumsPage;