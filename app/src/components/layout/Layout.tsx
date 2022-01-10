import * as React from "react";
import { FC } from "react";
import "react-toastify/dist/ReactToastify.css";
import Header from "./Header";
import UploadProgress from "../misc/UploadProgress";
import { HeaderSettings } from "../../models/HeaderSettings";
import uploadHelper from "../../helpers/UploadHelper";

interface Props {
	header: HeaderSettings,
	children: React.ReactNode
}

const Layout: FC<Props> = (props: Props) => {
	const [, forceUpdateComponent] = React.useState<unknown>({});

	// The UploadProgress often 'hangs'; I can see the component being called with the correct values,
	// but the UI itself never updates. I don't understand why yet, but forcing an update whenever
	// uploadHelper notifies us of an update works around the issue.
	// TODO: Find real cause and fix properly.
	React.useEffect(() => {
		uploadHelper.subscribe({
			update: () => forceUpdateComponent({})
		});
	}, []);

	return <>
		<Header settings={props.header} />
		{props.children}
		<UploadProgress />
	</>;
};

export default Layout;