import * as React from "react";
import { PageBaseComponent, PageBaseComponentProps } from "./PageBaseComponent";
import ContentContainer from "../ContentContainer";
import appStateContext from "../../contexts/AppStateContext";
import ModalCreateCollection from "../modals/ModalCreateCollection";
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

		this.refreshCollections = this.refreshCollections.bind(this);

		this.state = {
			collections: [],
			newCollectionDialogOpen: false
		};
	}

	getHeaderActions(): JSX.Element | null {
		return  <React.Fragment>
			<button onClick={() => this.onCreateCollectionClick()} title="Create collection">
				New collection
			</button>
		</React.Fragment>;
	}

	getTitle(): string {
		return "Collections";
	}

	componentDidMount(): void {
		this.refreshCollections();
		super.componentDidMount();
	}

	refreshCollections(): void {
		PhotoService.getCollections()
			.then(collections => {
				this.setState({ collections });
			})
			.catch(console.error);
	}

	onCreateCollectionClick(): void {
		this.setState({
			newCollectionDialogOpen: true
		});
	}

	createCollection(title: string): void {
		PhotoService.createCollection(title)
			.then(() => this.refreshCollections())
			.catch(console.error)
			.finally(() => this.setState({newCollectionDialogOpen: false}));
	}

	render(): React.ReactNode {
		return (
			<ContentContainer paddingTop={true} className="collections">
				{this.state.collections.map(collection => {
					return <UserCollection
						key={collection.id}
						collection={collection}
						onCollectionUpdated={this.refreshCollections}
						onCollectionDeleted={this.refreshCollections}
						/>;
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

SharedPage.contextType = appStateContext;
export default SharedPage;