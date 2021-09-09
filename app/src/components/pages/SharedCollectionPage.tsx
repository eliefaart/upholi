import * as React from "react";
import { PageBaseComponent, PageBaseComponentProps } from "./PageBaseComponent";
import PhotoService from "../../services/PhotoService";
import ContentContainer from "../ContentContainer";
import appStateContext from "../../contexts/AppStateContext";
import CollectionView from "../CollectionView";
import Collection from "../../models/Collection";
import InputPassword from "../InputPassword";

interface CollectionPageBaseState {
	unauthorized: boolean,
	lastPasswordIncorrect: boolean,
	collection: Collection | null
}

class SharedCollectionPage extends PageBaseComponent<CollectionPageBaseState> {
	readonly collectionToken: string;

	constructor(props: PageBaseComponentProps) {
		super(props);

		this.collectionToken = props.match.params.token;

		this.state = {
			unauthorized: false,
			lastPasswordIncorrect: false,
			collection: null
		};
	}

	componentDidMount(): void {
		this.getCollection();
	}

	getCollection(): void {
		this.setState({
			unauthorized: false
		});

		PhotoService.getCollectionByShareToken(this.collectionToken)
			.then((collection) => this.setState({ collection }))
			.catch((response) => {
				if (response.status === 401) {
					this.setState({
						unauthorized: true
					});
				}
				else {
					console.error(response);
				}
			});
	}

	getTitle(): string {
		return this.state.collection
			? "Collection - " + this.state.collection.title
			: super.getTitle();
	}

	authenticate(password: string): void {
		if (password) {
			PhotoService.authenticateToCollectionByShareToken(this.collectionToken, password)
				.then(() => {
					this.setState({
						lastPasswordIncorrect: false
					});
					this.getCollection();
				})
				.catch(response => {
					if (response.status === 401) {
						this.setState({
							lastPasswordIncorrect: true
						});
					}
					else {
						console.error(response);
					}
				});
		}
		else {
			this.setState({
				lastPasswordIncorrect: false
			});
		}
	}

	render(): React.ReactNode {
		return (
			<ContentContainer>
				{/* Password input box */}
				{this.state.unauthorized && <InputPassword
					className="padding-top-50px"
					prompt="You need to provide a password to view this collection."
					onSubmitPassword={(password) => this.authenticate(password)}
					lastPasswordIncorrect={this.state.lastPasswordIncorrect}/>}

				{/* Collection view  */}
				{this.state.collection != null && <CollectionView
					collection={this.state.collection}/>}
			</ContentContainer>
		);
	}
}

SharedCollectionPage.contextType = appStateContext;
export default SharedCollectionPage;