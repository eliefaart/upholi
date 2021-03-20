import * as React from "react";
import AlbumInfo from "../models/AlbumInfo";
import Collection from "../models/Collection";
import PhotoService from "../services/PhotoService";
import CollectionSharingSettings from "./CollectionSharingSettings";
import { IconChevronDown, IconChevronUp, IconClose, IconCreate, IconDelete } from "./Icons";
import ModalAddAlbumToCollection from "./modals/ModalAddAlbumToCollection";
import ModalConfirmation from "./modals/ModalConfirmation";
import OrderableContent from "./OrderableContent";

interface Props {
	collection: Collection;
	onCollectionUpdated: () => void;
	onCollectionDeleted: () => void;
}

interface State {
	settingsOpened: boolean;
	confirmDeleteCollectionOpen: boolean;
	confirmRemoveAlbumFromCollectionOpen: boolean;
	addAlbumToCollectionDialogOpen: boolean;
	activeAlbum: AlbumInfo | null;
}

export default class UserCollection extends React.Component<Props, State> {
	constructor(props: Props) {
		super(props);

		this.toggleSettings = this.toggleSettings.bind(this);
		this.onClickDeleteCollection = this.onClickDeleteCollection.bind(this);
		this.onClickAddAlbumToCollection = this.onClickAddAlbumToCollection.bind(this);
		this.onAlbumOrderChanged = this.onAlbumOrderChanged.bind(this);

		this.state = {
			settingsOpened: false,
			confirmDeleteCollectionOpen: false,
			confirmRemoveAlbumFromCollectionOpen: false,
			addAlbumToCollectionDialogOpen: false,
			activeAlbum: null
		};
	}

	/**
	 * Toggles the settings pane of the collection
	 */
	toggleSettings() {
		this.setState(prevState => {
			return {
				settingsOpened: !prevState.settingsOpened
			};
		});
	}

	onClickDeleteCollection() {
		this.setState({confirmDeleteCollectionOpen: true});
	}

	onClickRemoveAlbumFromCollection(album: AlbumInfo) {
		this.setState({
			confirmRemoveAlbumFromCollectionOpen: true,
			activeAlbum: album
		});
	}

	onClickAddAlbumToCollection() {
		this.setState({
			addAlbumToCollectionDialogOpen: true
		});
	}

	onAlbumOrderChanged(movedItemKey: string, newPosition: number): void {
		console.log("onAlbumOrderChanged", movedItemKey, newPosition);
	}

	/**
	 * Delete the current collection
	 */
	deleteCollection() {
		PhotoService.deleteCollection(this.props.collection.id)
			.then(() => this.props.onCollectionDeleted())
			.catch(console.error)
			.finally(() => this.setState({confirmDeleteCollectionOpen: false}));
	}

	/**
	 * Remove an album from the collection
	 */
	removeAlbumFromCollection(albumId: string) {
		PhotoService.removeAlbumFromCollection(this.props.collection.id, albumId)
			.then(() => this.props.onCollectionUpdated())
			.catch(console.error)
			.finally(() => this.setState({confirmRemoveAlbumFromCollectionOpen: false}));
	}

	/**
	 * Add an album to the collection
	 * @param albumId
	 */
	addAlbumToCollection(albumId: string) {
		PhotoService.addAlbumToCollection(this.props.collection.id, albumId)
			.then(() => this.props.onCollectionUpdated())
			.catch(console.error)
			.finally(() => this.setState({addAlbumToCollectionDialogOpen: false}));
	}

	render() {
		return <div key={this.props.collection.id} className="collection">
			<div className="head">
				{/* Collection title and some actions/buttons */}
				<h2 className="title">{this.props.collection.title}</h2>
				<button className="iconOnly" onClick={this.toggleSettings} title="Collection sharing options">
					{this.state.settingsOpened && <IconChevronUp/>}
					{!this.state.settingsOpened && <IconChevronDown/>}
				</button>
				<button className="iconOnly"
					onClick={this.onClickDeleteCollection}
					title="Delete collection">
					<IconDelete/>
				</button>
			</div>

			<div className="body">
				<div className={"settings" + (this.state.settingsOpened ? " open" : "")}>
					{this.state.settingsOpened && <CollectionSharingSettings collection={this.props.collection} onOptionsChanged={() => this.props.onCollectionUpdated()}/>}
				</div>
				{this.state.settingsOpened && <hr/>}

				{/* Albums inside this collection */}
				<div className="">
					<OrderableContent
						className="collection-albums"
						onOrderChanged={this.onAlbumOrderChanged}>
						{this.props.collection.albums.map(album => {
							let albumThumbUrl = album.thumbPhotoId
								? "url('" + PhotoService.baseUrl() + "/photo/" + album.thumbPhotoId + "/thumb')"
								: "";

							return (<div key={album.id}
								className="album"
								style={{ backgroundImage: albumThumbUrl }}>
								<div className="albumContent">
									<span className="title">{album.title}</span>
									<button className="iconOnly" onClick={(event) => {
										event.stopPropagation();
										this.onClickRemoveAlbumFromCollection(album);
									}} title="Remove album from collection">
										<IconClose/>
									</button>
								</div>
							</div>);
						})}
					</OrderableContent>
					<button className="iconOnly" onClick={this.onClickAddAlbumToCollection} title="Add album to collection">
						<IconCreate/> TODO
					</button>
				</div>
			</div>


			{this.state.addAlbumToCollectionDialogOpen && <ModalAddAlbumToCollection
				isOpen={true}
				onRequestClose={() => this.setState({addAlbumToCollectionDialogOpen: false})}
				onAlbumSelected={(album) => this.addAlbumToCollection(album.id)}
				/>}
			{this.state.confirmDeleteCollectionOpen && <ModalConfirmation
				title="Delete collection"
				isOpen={true}
				onRequestClose={() => this.setState({confirmDeleteCollectionOpen: false, activeAlbum: null})}
				onOkButtonClick={() => this.deleteCollection()}
				okButtonText="Delete"
				confirmationText={"Collection '" + this.props.collection.title + "' will be deleted."}
				/>}

			{this.state.confirmRemoveAlbumFromCollectionOpen && <ModalConfirmation
				title="Remove album from collection"
				isOpen={true}
				onRequestClose={() => this.setState({confirmRemoveAlbumFromCollectionOpen: false, activeAlbum: null})}
				onOkButtonClick={() => this.removeAlbumFromCollection(this.state.activeAlbum!.id)}
				okButtonText="Remove"
				confirmationText={"Album '"
					+ this.state.activeAlbum!.title
					+ "' will be remove from collection '"
					+ this.props.collection.title + "'."}
				/>}
		</div>;
	}
}