import * as React from "react";
import { PageBaseComponent, PageBaseComponentProps } from "./PageBaseComponent";
import ContentContainer from "../ContentContainer"
import AppStateContext from "../../contexts/AppStateContext";
import ModalCreateCollection from "../modals/ModalCreateCollection"
import ModalAddAlbumToCollection from "../modals/ModalAddAlbumToCollection"
import ModalConfirmation from "../modals/ModalConfirmation"
import { IconCreate, IconDelete, IconShare, IconClose, IconChevronDown, IconChevronUp } from "../Icons";
import PhotoService from "../../services/PhotoService";
import Collection from "../../models/Collection";
import CollectionSharingSettings from "../CollectionSharingSettings";

interface SharedPageState {
	collections: Collection[],
	settingsOpenCollectionIds: string[],
	// Modal state
	newCollectionDialogOpen: boolean,
	addAlbumToCollectionDialogOpen: boolean,
	confirmDeleteCollectionOpen: boolean,
	confirmRemoveAlbumFromCollectionOpen: boolean,
	// Temp variables during confirm modals
	activeCollectionId: string | null,
	activeAlbumId: string | null,
}

class SharedPage extends PageBaseComponent<SharedPageState> {
	constructor(props: PageBaseComponentProps) {
		super(props);

		this.state = {
			collections: [],
			settingsOpenCollectionIds: [],
			newCollectionDialogOpen: false,
			addAlbumToCollectionDialogOpen: false,
			confirmDeleteCollectionOpen: false,
			confirmRemoveAlbumFromCollectionOpen: false,
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
		const fnSetCollections = (collections: Collection[]) => this.setState({ collections });

		PhotoService.getCollections()
			.then(fnSetCollections)
			.catch(console.error);
	}

	openAlbum(albumId: string) {
		this.context.history.push("/album/" + albumId);
	}

	deleteCollection(collectionId: string) {
		PhotoService.deleteCollection(collectionId)
			.then(() => this.refreshCollections())
			.catch(console.error)
			.finally(() => this.setState({confirmDeleteCollectionOpen: false}));
	}

	toggleSettings(collectionId: string) {
		let collectionIds = this.state.settingsOpenCollectionIds;
		const collectionIdIndex = collectionIds.indexOf(collectionId);
		const collectionIdOpened = collectionIdIndex !== -1;

		if (collectionIdOpened) {
			collectionIds.splice(collectionIdIndex, 1);
		}
		else {
			collectionIds.push(collectionId);
		}

		this.setState({
			settingsOpenCollectionIds: collectionIds
		});
	}

	addAlbumToCollection(collectionId: string, albumId: string) {
		PhotoService.addAlbumToCollection(collectionId, albumId)
			.then(() => this.refreshCollections())
			.catch(console.error)
			.finally(() => this.setState({addAlbumToCollectionDialogOpen: false}));
	}

	removeAlbumFromCollection(collectionId: string, albumId: string) {
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

	createCollection(title: string) {
		PhotoService.createCollection(title)
			.then((id) => this.refreshCollections())
			.catch(console.error)
			.finally(() => this.setState({newCollectionDialogOpen: false}));
	}

	onClickDeleteCollection(collectionId: string) {
		this.setState({
			confirmDeleteCollectionOpen: true,
			activeCollectionId: collectionId
		});
	}

	onAddAlbumToCollectionClick(collectionId: string) {
		this.setState({
			addAlbumToCollectionDialogOpen: true,
			activeCollectionId: collectionId
		});
	}

	onRemoveAlbumFromCollectionClick(collectionId: string, albumId: string) {
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
					{this.state.collections.map(collection => {
						const settingsOpened = this.state.settingsOpenCollectionIds.indexOf(collection.id) !== -1;

						// Collection container
						return <div key={collection.id} className="collection">
							<div className="head">
								{/* Collection title and some actions/buttons */}
								<h2 className="title">{collection.title}</h2>
								<button className="iconOnly" onClick={() => this.toggleSettings(collection.id)} title="Collection sharing options">
									{settingsOpened && <IconChevronUp/>}
									{!settingsOpened && <IconChevronDown/>}
								</button>
								<button className="iconOnly" onClick={() => this.onClickDeleteCollection(collection.id)} title="Delete collection">
									<IconDelete/>
								</button>
							</div>

							<div className="body">
								<div className={"settings" + (settingsOpened ? " open" : "")}>
									{settingsOpened && <CollectionSharingSettings collection={collection} onOptionsChanged={() => this.refreshCollections()}/>}
								</div>
								{settingsOpened && <hr/>}

								{/* Albums inside this collection */}
								<div className="collection-albums">
									{collection.albums.map(album => {
										let albumThumbUrl = album.thumbPhotoId
											? "url('" + PhotoService.baseUrl() + "/photo/" + album.thumbPhotoId + "/thumb')"
											: "";

										return (<div key={album.id}
											className="album"
											style={{ backgroundImage: albumThumbUrl }}
											onClick={() => this.openAlbum(album.id)}>
											<div className="albumContent">
												<span className="title">{album.title}</span>
												<button className="iconOnly" onClick={() => this.onRemoveAlbumFromCollectionClick(collection.id, album.id)} title="Remove album from collection">
													<IconClose/>
												</button>
											</div>
										</div>);
									})}
									<button className="" onClick={() => this.onAddAlbumToCollectionClick(collection.id)} title="Add album to collection">
										<IconCreate/> Add album
									</button>
								</div>
							</div>
						</div>;
					})}
				</div>

				{this.state.addAlbumToCollectionDialogOpen && <ModalAddAlbumToCollection
					isOpen={true}
					onRequestClose={() => this.setState({addAlbumToCollectionDialogOpen: false})}
					onAlbumSelected={(album) => this.addAlbumToCollection(this.state.activeCollectionId!, album.id)}
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
					onOkButtonClick={() => this.deleteCollection(this.state.activeCollectionId!)}
					okButtonText="Delete"
					confirmationText={"Collection '" + activeCollection!.title + "' will be deleted."}
					/>}
				{this.state.confirmRemoveAlbumFromCollectionOpen && <ModalConfirmation
					title="Remove album from collection"
					isOpen={true}
					onRequestClose={() => this.setState({confirmRemoveAlbumFromCollectionOpen: false})}
					onOkButtonClick={() => this.removeAlbumFromCollection(this.state.activeCollectionId!, this.state.activeAlbumId!)}
					okButtonText="Remove"
					confirmationText={"Album '"
						+ activeCollection!.title
						+ "' will be remove from collection '"
						+ activeAlbum!.title + "'."}
					/>}
			</ContentContainer>
		);
	}
}

SharedPage.contextType = AppStateContext;
export default SharedPage;