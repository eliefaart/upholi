import React from "react";
import PageBaseComponent from "../components/PageBaseComponent.jsx";
import ContentContainer from "../components/ContentContainer.tsx"
import AppStateContext from "../contexts/AppStateContext.ts";
import ModalCreateCollection from "../components/ModalCreateCollection.jsx"
import ModalAddAlbumToCollection from "../components/ModalAddAlbumToCollection.jsx"
import ModalConfirmation from "../components/ModalConfirmation.jsx"
import ModalShareCollection from "../components/ModalShareCollection.jsx"
import { IconCreate, IconDelete, IconShare, IconClose } from "../components/Icons.tsx";
import PhotoService from "../services/PhotoService.ts";

class CollectionsPage extends PageBaseComponent {
	constructor(props) {
		super(props);

		this.state = {
			collections: [],
			// Modal state
			collectionSharingOptionsDialoOpen: false,
			newCollectionDialogOpen: false,
			addAlbumToCollectionDialogOpen: false,
			confirmDeleteCollectionOpen: false,
			confirmRemoveAlbumFromCollectionOpen: false,
			// Temp variables during confirm modals
			activeCollectionId: null,
			activeAlbumId: null,
		}
	}

	getHeaderActions() {
		return  (<React.Fragment>
			<button onClick={() => this.onCreateCollectionClick()} title="Create collection">
				New collection
			</button>
		</React.Fragment>);
	}

	getTitle() {
		return "Collections";
	}

	componentDidMount() {
		this.refreshCollections();
		super.componentDidMount();
	}

	refreshCollections() {
		const fnSetCollections = (collections) => this.setState({ collections });

		PhotoService.getCollections()
			.then(fnSetCollections)
			.catch(console.error);
	}

	openAlbum(albumId) {
		this.context.history.push("/album/" + albumId);
	}

	openCollection(collectionId) {
		this.context.history.push("/collection/" + collectionId);
	}

	createCollection(title) {
		PhotoService.createCollection(title)
			.then(() => this.refreshCollections())
			.catch(console.error)
			.finally(() => this.setState({newCollectionDialogOpen: false}))
	}

	deleteCollection(collectionId) {
		PhotoService.deleteCollection(collectionId)
			.then(() => this.refreshCollections())
			.catch(console.error)
			.finally(() => this.setState({confirmDeleteCollectionOpen: false}));
	}

	addAlbumToCollection(collectionId, albumId) {
		PhotoService.addAlbumToCollection(collectionId, albumId)
			.then(() => this.refreshCollections())
			.catch(console.error)
			.finally(() => this.setState({addAlbumToCollectionDialogOpen: false}));
	}

	removeAlbumFromCollection(collectionId, albumId) {
		PhotoService.removeAlbumFromCollection(collectionId, albumId)
			.then(() => this.refreshCollections())
			.catch(console.error)
			.finally(() => this.setState({confirmRemoveAlbumFromCollectionOpen: false}));
	}

	onCreateCollectionClick() {
		this.setState({
			newCollectionDialogOpen: true
		});
	}

	createCollection(title) {
		PhotoService.createCollection(title)
			.then((id) => this.refreshCollections())
			.catch(console.error)
			.finally(() => this.setState({newCollectionDialogOpen: false}));
	}

	openShareModal(collectionId) {
		this.setState({
			collectionSharingOptionsDialoOpen: true,
			activeCollectionId: collectionId
		});
	}

	onClickDeleteCollection(collectionId) {
		this.setState({
			confirmDeleteCollectionOpen: true,
			activeCollectionId: collectionId
		});
	}

	onAddAlbumToCollectionClick(collectionId) {
		this.setState({
			addAlbumToCollectionDialogOpen: true,
			activeCollectionId: collectionId
		});
	}

	onRemoveAlbumFromCollectionClick(collectionId, albumId) {
		this.setState({
			confirmRemoveAlbumFromCollectionOpen: true,
			activeCollectionId: collectionId,
			activeAlbumId: albumId
		});
	}

	render() {
		const activeCollection = this.state.collections.find(col => col.id === this.state.activeCollectionId);
		const activeAlbum = activeCollection && activeCollection.albums.find(alb => alb.id === this.state.activeAlbumId);

		return (
			<ContentContainer paddingTop={true}>
				<div className="collections">
					{this.state.collections.map(collection => (
						// Collection container
						<div key={collection.id} className="collection">
							<div className="head">
								{/* Collection title and some actions/buttons */}
								<h2 className="title" onClick={() => this.openCollection(collection.id)}>{collection.title}</h2>
								<button className={"iconOnly" + (collection.sharing.shared ? " shared" : "")} onClick={() => this.openShareModal(collection.id)} title="Collection sharing options">
									<IconShare/>
								</button>
								<button className="iconOnly" onClick={() => this.onAddAlbumToCollectionClick(collection.id)} title="Add album to collection">
									<IconCreate/>
								</button>
								<button className="iconOnly" onClick={() => this.onClickDeleteCollection(collection.id)} title="Delete collection">
									<IconDelete/>
								</button>
							</div>
							<div className="body">
								{/* Albums inside this collection */}
								{collection.albums.map(album => {
									let albumThumbUrl = "url('" + PhotoService.baseUrl() + "/photo/" + album.thumbPhotoId + "/thumb')";

									return (<div key={album.id}
										className="album"
										style={{ backgroundImage: !!album.thumbPhotoId && albumThumbUrl }}
										onClick={() => this.openAlbum(album.id)}>
										<div className="albumContent">
											<span className="title">{album.title}</span>
											<button className="iconOnly" onClick={() => this.onRemoveAlbumFromCollectionClick(collection.id, album.id)} title="Remove album from collection">
												<IconClose/>
											</button>
										</div>
									</div>);
								})}
							</div>
						</div>
					))}
				</div>

				{this.state.collectionSharingOptionsDialoOpen && <ModalShareCollection
					isOpen={true}
					onRequestClose={() => this.setState({collectionSharingOptionsDialoOpen: false})}
					collection={activeCollection}
					onOptionsChanged={() => this.refreshCollections()}
					/>}
				{this.state.addAlbumToCollectionDialogOpen && <ModalAddAlbumToCollection
					isOpen={true}
					onRequestClose={() => this.setState({addAlbumToCollectionDialogOpen: false})}
					onAlbumSelected={(album) => this.addAlbumToCollection(this.state.activeCollectionId, album.id)}
					/>}
				{this.state.newCollectionDialogOpen && <ModalCreateCollection
					isOpen={true}
					onRequestClose={() => this.setState({newCollectionDialogOpen: false})}
					onOkButtonClick={(title) => this.createCollection(title)}
					/>}
				{this.state.confirmDeleteCollectionOpen && <ModalConfirmation
					title="Delete collection"
					isOpen={true}
					onRequestClose={() => this.setState({confirmDeleteCollectionOpen: false})}
					onOkButtonClick={() => this.deleteCollection(this.state.activeCollectionId)}
					okButtonText="Delete"
					confirmationText={"Collection '" + activeCollection.title + "' will be deleted."}
					/>}
				{this.state.confirmRemoveAlbumFromCollectionOpen && <ModalConfirmation
					title="Remove album from collection"
					isOpen={true}
					onRequestClose={() => this.setState({confirmRemoveAlbumFromCollectionOpen: false})}
					onOkButtonClick={() => this.removeAlbumFromCollection(this.state.activeCollectionId, this.state.activeAlbumId)}
					okButtonText="Remove"
					confirmationText={"Album '"
						+ activeCollection.title
						+ "' will be remove from collection '"
						+ activeAlbum.title + "'."}
					/>}
			</ContentContainer>
		);
	}
}

CollectionsPage.contextType = AppStateContext;
export default CollectionsPage;