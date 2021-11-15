import * as React from "react";
import { FC } from "react";
import "react-toastify/dist/ReactToastify.css";
import Header from "./Header";
import UploadProgress from "../misc/UploadProgress";
import { HeaderSettings } from "../../models/HeaderSettings";

interface Props {
	header: HeaderSettings,
	children: React.ReactNode
}

const Layout: FC<Props> = (props: Props) => {
	return <>
		<Header settings={props.header}/>
		{props.children}
		<UploadProgress/>
	</>;
};

export default Layout;