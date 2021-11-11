import * as React from "react";
import { FC } from "react";
import "react-toastify/dist/ReactToastify.css";
import Header from "./Header";
import UploadProgress from "../misc/UploadProgress";

const Layout: FC = (props) => {
	return <>
		<Header/>
		{props.children}
		<UploadProgress/>
	</>;
};

export default Layout;