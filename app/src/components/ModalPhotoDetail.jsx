import React from "react";
import Modal from "./Modal.jsx";
import PhotoService from "../services/PhotoService";
import PhotoDetail from "../components/PhotoDetail.jsx";
import { IconDownload } from "../components/Icons.jsx";

class ModalPhotoDetail extends React.Component {

	constructor(props) {
		super(props);

		this.state = {};
	}

	componentDidUpdate(prevProps) {
		if (this.props.photoId && prevProps.photoId !== this.props.photoId) {
			const fnOnPhotoDataReceived = (photo) => {
				this.setState({ photo });
			};

			PhotoService.getPhotoInfo(this.props.photoId)
				.then(fnOnPhotoDataReceived)
				.catch(console.error);
		}
	}

	render() {
		if (!this.props.photoId) {
			return null;
		}

		const photoBaseUrl = PhotoService.baseUrl() + "/photo/" + this.props.photoId;
		const previewUrl = photoBaseUrl + "/preview";
		const downloadUrl = photoBaseUrl + "/original";

		const headerActions = <a className="iconOnly asButton" href={downloadUrl} download title="Download">
			<IconDownload/>
		</a>;

		return (
			<Modal
				title=""
				isOpen={this.props.isOpen}
				onRequestClose={this.props.onRequestClose}
				className={this.props.className + " modalPhotoDetail fullscreen"}
				headerActions={headerActions}
				okButtonText={null}
			>
				<PhotoDetail src={previewUrl} exif={!!this.state.photo ? this.state.photo.exif : null} />
			</Modal>
		);
	}
}

export default ModalPhotoDetail;