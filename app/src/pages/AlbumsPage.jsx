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
		const headerContextMenuActions = (<div>
			{<button onClick={(e) => this.onCreateAlbumClick()}>Create album</button>}
		</div>);

		return (
			<PageLayout headerContextMenuActions={headerContextMenuActions}>
				<Albums/>

				<ModalCreateAlbum
					isOpen={this.state.newAlbumDialogOpen}
					onRequestClose={() => this.setState({newAlbumDialogOpen: false})}/>
			</PageLayout>
		);
	}
}

export default AlbumsPage;