import * as React from "react";
import { PageBaseComponent, PageBaseComponentProps } from "./PageBaseComponent";
import Content from "../layout/Content";
import appStateContext from "../../contexts/AppStateContext";
import upholiService from "../../services/UpholiService";
import { Share } from "../../models/Share";
import { AlbumNew } from "../../models/Album";
import CopyUrl from "../misc/CopyUrl";

interface SharedPageState {
	shares: Share[],
	albums: AlbumNew[]
}

class SharedPage extends PageBaseComponent<SharedPageState> {
	constructor(props: PageBaseComponentProps) {
		super(props);

		this.deleteShare = this.deleteShare.bind(this);

		upholiService.getAlbums()
			.then(albums => {
				this.setState({
					albums
				});
			});

		this.state = {
			shares: [],
			albums: []
		};
	}

	componentDidMount(): void {
		this.context.headerActions = this.getHeaderActions();
		this.context.headerContextMenu = this.getHeaderContextMenu();

		upholiService.getShares()
			.then(shares => {
				this.setState({
					shares
				});
			});
	}

	getHeaderActions(): JSX.Element | null {
		return <></>;
	}

	getTitle(): string {
		return "Shared";
	}

	deleteShare(share: Share): void {
		upholiService.deleteShare(share.id)
			.then(() => {
				this.setState((prevState) => {
					prevState.shares.filter(s => s.id !== share.id);
					return {
						shares: prevState.shares.filter(s => s.id !== share.id)
					};
				});
			})
			.catch(console.error);
	}

	render(): React.ReactNode {
		const history = this.context.history;

		return (
			<Content paddingTop={true} className="shares">
				{this.state.shares.map(share => {
					const shareUrl = document.location.origin + "/s/" + share.id;
					const shareAlbum = this.state.albums.find(album => album.id === share.data.album.albumId);

					return <div key={share.id} className="share">
						<div className="head">
							<h2 onClick={() => history.push("/album/" + shareAlbum?.id)}>
								{shareAlbum?.title}
							</h2>
						</div>
						<div className="body">
							<CopyUrl shareUrl={shareUrl}/>
							<div className="actions">
							<button onClick={() => this.deleteShare(share)}>
								Delete share
							</button>
							</div>
						</div>
					</div>;
				})}
			</Content>
		);
	}
}

SharedPage.contextType = appStateContext;
export default SharedPage;