import * as React from "react";
import { PageBaseComponent, PageBaseComponentProps } from "./PageBaseComponent";
import ContentContainer from "../ContentContainer"
import AppStateContext from "../../contexts/AppStateContext";
import ModalCreateCollection from "../modals/ModalCreateCollection"
import PhotoService from "../../services/PhotoService";
import Collection from "../../models/Collection";
import UserCollection from "../UserCollection";

interface SharedPageState {
	collections: Collection[],
	newCollectionDialogOpen: boolean,
}

class SharedPage extends PageBaseComponent<SharedPageState> {
	constructor(props: PageBaseComponentProps) {
		super(props);

		this.onAlbumOrderChanged = this.onAlbumOrderChanged.bind(this);
		this.refreshCollections = this.refreshCollections.bind(this);

		this.state = {
			collections: [],
			newCollectionDialogOpen: false
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
		PhotoService.getCollections()
			.then(collections => this.setState({ collections }))
			.catch(console.error);
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

	onAlbumOrderChanged(movedItem: unknown, newPosition: number): void {
		console.log("onAlbumOrderChanged", movedItem, newPosition);
	}

	render() {
		return (
			<ContentContainer paddingTop={true} className="collections">
				{this.state.collections.map(collection => {
					return <UserCollection
						key={collection.id}
						collection={collection}
						onCollectionUpdated={this.refreshCollections}
						onCollectionDeleted={this.refreshCollections}
						/>
				})}

				{this.state.newCollectionDialogOpen && <ModalCreateCollection
					isOpen={true}
					onRequestClose={() => this.setState({newCollectionDialogOpen: false})}
					onOkButtonClick={(title) => this.createCollection(title)}
					/>}
			</ContentContainer>
		);
	}
}

SharedPage.contextType = AppStateContext;
export default SharedPage;