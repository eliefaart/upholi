import * as React from "react";
import { PageBaseComponent, PageBaseComponentProps } from "./PageBaseComponent";
import ContentContainer from "../misc/ContentContainer";
import appStateContext from "../../contexts/AppStateContext";
import { Album } from "../../models/Album";
import InputPassword from "../misc/InputPassword";
import upholiService from "../../services/UpholiService";
import AlbumView from "../AlbumView";

interface State {
	authorized: boolean,
	lastPasswordIncorrect: boolean,
	album: Album | null
}

class SharedAlbumPage extends PageBaseComponent<State> {
	readonly token: string;

	constructor(props: PageBaseComponentProps) {
		super(props);

		this.token = props.match.params.token;

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

	tryUnlockShare(password: string): void {
		if (password) {
			upholiService.getAlbumFromShare(this.token, password)
				.then(album => {
					this.setState({
						authorized: true,
						lastPasswordIncorrect: false,
						album
					});
				})
				.catch(error => {
					if (error) {
						console.log(error);
					}

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
		return (
			<ContentContainer>
				{/* Password input box */}
				{!this.state.authorized && <InputPassword
					className="padding-top-50px"
					prompt="You need to provide a password to access this share."
					onSubmitPassword={(password) => this.tryUnlockShare(password)}
					lastPasswordIncorrect={this.state.lastPasswordIncorrect}/>}

				{this.state.album != null && <AlbumView album={this.state.album} />}
			</ContentContainer>
		);
	}
}

SharedAlbumPage.contextType = appStateContext;
export default SharedAlbumPage;