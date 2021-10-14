import * as React from "react";
import { PageBaseComponent, PageBaseComponentProps } from "./PageBaseComponent";
import ContentContainer from "../ContentContainer";
import appStateContext from "../../contexts/AppStateContext";
import upholiService from "../../services/UpholiService";

interface SharedPageState {

}

class SharedPage extends PageBaseComponent<SharedPageState> {
	constructor(props: PageBaseComponentProps) {
		super(props);

		this.state = { };
	}

	componentDidMount(): void {
		upholiService.getShares()
			.then(console.log);
	}

	getHeaderActions(): JSX.Element | null {
		return <></>;
	}

	getTitle(): string {
		return "Shared";
	}

	render(): React.ReactNode {
		return (
			<ContentContainer paddingTop={true}>
			</ContentContainer>
		);
	}
}

SharedPage.contextType = appStateContext;
export default SharedPage;