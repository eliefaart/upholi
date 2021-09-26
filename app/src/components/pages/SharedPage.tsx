import * as React from "react";
import { PageBaseComponent, PageBaseComponentProps } from "./PageBaseComponent";
import ContentContainer from "../ContentContainer";
import appStateContext from "../../contexts/AppStateContext";

interface SharedPageState {
}

class SharedPage extends PageBaseComponent<SharedPageState> {
	constructor(props: PageBaseComponentProps) {
		super(props);

		this.state = { };
	}

	getHeaderActions(): JSX.Element | null {
		return  <React.Fragment></React.Fragment>;
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