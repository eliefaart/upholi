import * as React from "react";
import Modal from "./Modal";
import PhotoService from "../services/PhotoService";
import PhotoDetail from "../components/PhotoDetail";
import { IconDownload } from "../components/Icons";
import ModalPropsBase from "../entities/ModalPropsBase";
import Photo from "../entities/Photo";

interface ModalPhotoDetailProps extends ModalPropsBase {
	photoId: string
}

interface ModalPhotoDetailState {
	photo: Photo | null
}

class ModalPhotoDetail extends React.Component<ModalPhotoDetailProps, ModalPhotoDetailState> {

	isRequestingPhotoId: string | null;

	constructor(props: ModalPhotoDetailProps) {
		super(props);

		this.isRequestingPhotoId = null;

		this.state = {
			photo: null
		};
	}

	componentDidMount() {
		this.getPhotoInfo();
	}

	componentDidUpdate(prevProps: ModalPhotoDetailProps) {
		if (this.props.photoId && this.isRequestingPhotoId !== this.props.photoId && (this.state.photo == null || this.state.photo?.id !== prevProps.photoId)) {
			this.getPhotoInfo();
		}
	}

	getPhotoInfo() {
		if (this.props.photoId) {
			const fnOnPhotoDataReceived = (photo: Photo) => {
				this.setState({ photo });
			};

			this.isRequestingPhotoId = this.props.photoId;
			PhotoService.getPhotoInfo(this.props.photoId)
				.then(fnOnPhotoDataReceived)
				.catch(console.error)
				.finally(() => this.isRequestingPhotoId = null);
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