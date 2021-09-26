import * as React from "react";
import { PageBaseComponent, PageBaseComponentProps } from "./PageBaseComponent";
import ContentContainer from "../ContentContainer";
import appStateContext from "../../contexts/AppStateContext";
import Album from "../../models/Album";
import InputPassword from "../InputPassword";
import upholiService from "../../services/UpholiService";

interface State {
	authorized: boolean,
	lastPasswordIncorrect: boolean,
	album: Album | null
}

class SharedCollectionPage extends PageBaseComponent<State> {
	readonly collectionToken: string;

	constructor(props: PageBaseComponentProps) {
		super(props);

		this.collectionToken = props.match.params.token;

		this.state = {
			authorized: false,
			lastPasswordIncorrect: false,
			album: null
		};
	}

	getTitle(): string {
		return this.state.album
			? "Album - " + this.state.album.title
			: super.getTitle();
	}

	authenticate(password: string): void {
		if (password) {
			upholiService.getAlbumByShareToken(this.collectionToken, password)
				.then(album => {
					this.setState({
						authorized: true,
						lastPasswordIncorrect: false,
						album
					});
				})
				.catch(error => {
					console.log(error);
					this.setState({
						lastPasswordIncorrect: true
					});
				});
		}
		else {
			this.setState({
				lastPasswordIncorrect: false
			});
		}
	}

	render(): React.ReactNode {
		console.log(this.state.album);
		return (
			<ContentContainer>
				{/* Password input box */}
				{!this.state.authorized && <InputPassword
					className="padding-top-50px"
					prompt="You need to provide a password to access this content."
					onSubmitPassword={(password) => this.authenticate(password)}
					lastPasswordIncorrect={this.state.lastPasswordIncorrect}/>}

				{/* Album view  */}
				{this.state.album != null && (
					<div className="topBar">
						<h1>{this.state.album.title}</h1>
					</div>
				)}
			</ContentContainer>
		);
	}
}

SharedCollectionPage.contextType = appStateContext;
export default SharedCollectionPage;