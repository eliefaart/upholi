import * as React from "react";
import { PageBaseComponent, PageBaseComponentProps } from "./PageBaseComponent";
import ContentContainer from "../ContentContainer";
import appStateContext from "../../contexts/AppStateContext";
import upholiService from "../../services/UpholiService";
import { Share } from "../../models/Share";

interface SharedPageState {
	shares: Share[]
}

class SharedPage extends PageBaseComponent<SharedPageState> {
	constructor(props: PageBaseComponentProps) {
		super(props);

		this.deleteShare = this.deleteShare.bind(this);

		this.state = {
			shares: []
		};
	}

	componentDidMount(): void {
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
		return (
			<ContentContainer paddingTop={true}>
				{this.state.shares.map(share => {
					return <div key={share.id} className="share">
						<span>{share.type}</span>
						<a href={`/s/${share.id}`}>{share.id}</a>
						<button onClick={() => this.deleteShare(share)}>Delete</button>
					</div>;
				})}
			</ContentContainer>
		);
	}
}

SharedPage.contextType = appStateContext;
export default SharedPage;